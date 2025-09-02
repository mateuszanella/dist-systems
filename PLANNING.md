# Planning of the distributed system task

## Mysql

Rules:
    - Não pode ter PK, FK, indexes.
    - Não pode ter procedures, triggers, etc.
    - Não há auto increment.
    - O sistema deve lidar com concorrência, e para isso locks podem ser utilizados.

### Tabelas

**table `status`**
- `id int`: Atuará como um contador global para gerar IDs únicos para os eventos.

**table `events`**
- `id int`: O ID único do evento.
- `value varchar(255) null`: O conteúdo do evento. `NULL` indica que o evento está pendente de processamento.

## Estratégia de Geração de ID

Como não há auto-increment, a tabela `status` será usada para gerar IDs. O fluxo será:
1. Iniciar uma transação.
2. Executar `SELECT id FROM status FOR UPDATE;` para bloquear a linha e evitar que outras transações leiam o valor antigo.
3. Ler o `id` atual.
4. Executar `UPDATE status SET id = id + 1;`.
5. O novo ID a ser usado é o `id` lido + 1.
6. A transação pode então continuar (para inserir o evento) e ser commitada, liberando o lock.

## Web Apps (Golang e Rust)

### `POST /events` (Síncrono)

Este endpoint cria um evento já processado.

1. Inicia uma transação no banco de dados.
2. Obtém um novo ID único utilizando a estratégia de geração de ID descrita acima.
3. Gera um valor aleatório (uma palavra ou string).
4. Executa `INSERT INTO events (id, value) VALUES (:new_id, :generated_value);`.
5. Commita a transação.
6. Retorna o evento completo: `{ "id": new_id, "value": "generated_value" }`.

### `POST /events/async` (Assíncrono)

Este endpoint enfileira um evento para ser processado por um worker.

1. Inicia uma transação no banco de dados.
2. Obtém um novo ID único utilizando a estratégia de geração de ID.
3. Executa `INSERT INTO events (id, value) VALUES (:new_id, NULL);`. O `NULL` sinaliza que o evento está pendente.
4. Commita a transação.
5. Retorna o ID do evento criado: `{ "id": new_id }`.

### Outros Endpoints

- `GET /events`: Retorna a contagem de todos os eventos na tabela `events`.
- `GET /events/{id}`: Retorna um evento específico pelo seu `id`.

## Apps Standalone (Workers)

Os workers são responsáveis por processar os eventos pendentes. Eles devem operar de forma robusta para evitar race conditions.

### Lógica de Polling e Processamento

Cada worker executará um loop contínuo com a seguinte lógica:

1. **Iniciar Transação**: Tudo começa com uma nova transação.
2. **Selecionar e Bloquear Evento**: Executar a query:
   ```sql
   SELECT id FROM events WHERE value IS NULL LIMIT 1 FOR UPDATE SKIP LOCKED;
   ```
   - `FOR UPDATE`: Bloqueia a linha selecionada. Apenas a transação atual poderá modificá-la.
   - `SKIP LOCKED`: Essencial para concorrência. Se uma linha já estiver bloqueada por outro worker, a query irá ignorá-la e tentar encontrar a próxima linha livre. Isso evita que workers fiquem esperando e melhora a vazão.
3. **Processar o Evento**:
   - **Se um `id` for retornado**:
     a. O worker agora tem um "lock" exclusivo sobre este evento.
     b. Gerar o valor (palavra/string).
     c. Executar `UPDATE events SET value = :generated_value WHERE id = :locked_id;`.
     d. **Commitar a transação**. Isso libera o lock e torna a atualização visível para todos.
   - **Se nenhum `id` for retornado**:
     a. Significa que não há eventos pendentes ou todos estão bloqueados por outros workers.
     b. **Rollback** da transação (uma boa prática, embora nada tenha sido alterado).
     c. O worker deve esperar por um curto período (ex: 1 segundo com um pouco de jitter, `sleep(1 + rand(0, 0.5))`) antes de tentar novamente. Isso evita sobrecarregar o banco de dados com queries vazias.

Este design garante que cada evento pendente seja processado por exatamente um worker, de forma segura e concorrente.

# Sistema Distribuído com Go e MySQL

Este projeto demonstra um sistema distribuído simples com APIs web em Go e Rust e workers que processam eventos de forma assíncrona, utilizando MySQL como banco de dados. O sistema foi projetado com restrições específicas, como a ausência de PKs, FKs e índices, e o uso de locks para concorrência.

## Pré-requisitos

Certifique-se de ter o Docker e o Docker Compose instalados em sua máquina.

## Como Rodar o Projeto

1.  **Clone o repositório** (se ainda não o fez):

    ```bash
    git clone <URL_DO_SEU_REPOSITORIO>
    cd dist-systems
    ```

2.  **Construa e inicie os serviços**:

    Na raiz do projeto, execute o seguinte comando para construir as imagens e iniciar todos os contêineres (MySQL, APIS e Workers):

    ```bash
    docker-compose up --build
    ```

    Aguarde até que todos os serviços estejam saudáveis. Você pode verificar o status com `docker-compose ps`.

## Endpoints da API (Go)

A API estará disponível em `http://localhost:8080`.

### 1. Criar Evento Síncrono (`POST /events`)

Cria um evento e aguarda o processamento do worker antes de retornar a resposta completa. O `value` do evento será gerado pelo worker.

```bash
curl -X POST http://localhost:8080/events
```

Exemplo de Resposta:

```json
{
  "id": 1,
  "value": "palavra_gerada_pelo_worker"
}
```

### 2. Criar Evento Assíncrono (`POST /events/async`)

Cria um evento com `value` nulo e retorna imediatamente o ID. O processamento do `value` será feito posteriormente por um worker.

```bash
curl -X POST http://localhost:8080/events/async
```

Exemplo de Resposta:

```json
{
  "id": 2
}
```

### 3. Obter Contagem Total de Eventos (`GET /events`)

Retorna a contagem total de eventos criados no sistema.

```bash
curl http://localhost:8080/events
```

Exemplo de Resposta:

```json
{
  "count": 2
}
```

### 4. Obter Evento por ID (`GET /events/{id}`)

Retorna os detalhes de um evento específico pelo seu ID.

```bash
curl http://localhost:8080/events/1
```

Exemplo de Resposta (para um evento processado):

```json
{
  "id": 1,
  "value": "palavra_gerada_pelo_worker"
}
```

Exemplo de Resposta (para um evento ainda não processado - `value` pode estar ausente ou nulo):

```json
{
  "id": 2
}
```

## Workers

Os workers rodam em segundo plano, fazendo polling no banco de dados por eventos com `value` nulo. Ao encontrar um, eles o bloqueiam, geram um `value` (uma palavra em português do arquivo `data/words.txt`) e atualizam o evento no banco de dados. Cada processamento simula um trabalho de 100ms.

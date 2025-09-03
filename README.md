# Sistema Distribuído com Go, Node.js e MySQL (com Nginx Load Balancer)

Este projeto demonstra um sistema distribuído simples com APIs web em Go e Node.js, balanceadas por um Nginx, e workers que processam eventos de forma assíncrona, utilizando MySQL como banco de dados. O sistema foi projetado com restrições específicas, como a ausência de PKs, FKs e índices, e o uso de locks para concorrência.

## Pré-requisitos

Certifique-se de ter o Docker e o Docker Compose instalados em sua máquina.

## Como Rodar o Projeto

1.  **Clone o repositório** (se ainda não o fez):

    ```bash
    git clone <URL_DO_SEU_REPOSITORIO>
    cd dist-systems
    ```

2.  **Construa e inicie os serviços**:

    Na raiz do projeto, execute o seguinte comando para construir as imagens e iniciar todos os contêineres (MySQL, Nginx, APIs Go e Node.js, e Workers Go e Node.js):

    ```bash
    docker-compose up --build
    ```

    Aguarde até que todos os serviços estejam saudáveis. Você pode verificar o status com `docker-compose ps`.

    **Observação**: O Nginx atuará como um balanceador de carga, distribuindo as requisições entre as APIs Go e Node.js.

## Endpoints da API (via Nginx Load Balancer)

As requisições para a API devem ser feitas para `http://localhost`. O Nginx irá balancear as requisições entre as APIs Go e Node.js.

### 1. Criar Evento Síncrono (`POST /events`)

Cria um evento e aguarda o processamento do worker (Go ou Node.js) antes de retornar a resposta completa. O `value` do evento será gerado pelo worker.

```bash
curl -X POST http://localhost/events
```

Exemplo de Resposta:

```json
{
  "id": 1,
  "value": "palavra_gerada_pelo_worker"
}
```

### 2. Criar Evento Assíncrono (`POST /events/async`)

Cria um evento com `value` nulo e retorna imediatamente o ID. O processamento do `value` será feito posteriormente por um worker (Go ou Node.js).

```bash
curl -X POST http://localhost/events/async
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
curl http://localhost/events
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
curl http://localhost/events/1
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

Os workers (Go e Node.js) rodam em segundo plano, fazendo polling no banco de dados por eventos com `value` nulo. Ao encontrar um, eles o bloqueiam, geram um `value` (uma palavra em português do arquivo `data/words.txt`) e atualizam o evento no banco de dados. Cada processamento simula um trabalho de 100ms.

## Serviços Rust (WIP - Work In Progress)

Os serviços da aplicação Rust (`api-rust` e `worker-rust`) estão presentes no `docker-compose.yml` mas estão comentados. Eles representam uma futura implementação alternativa para a API e o worker. Para ativá-los, descomente as seções correspondentes no `docker-compose.yml`.
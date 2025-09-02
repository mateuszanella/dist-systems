# Planning of the distributed system task



## Mysql

Rules: 
    - N pode ter PK, FK, indexes
    - n pode procedures, triggers, etc.
    - n há auto increment

table status
    id int

table events
    id int
    value varchar(255) null

## 2 Web Apps

Contratos:
GET /events # retorna o count dos eventos
GET /events/{id} # retorna o evento

POST /events 
    - cria o evento

POST /events/async
    - cria o evento de forma async, com value null para os apps standalo processar, retorna o id

- tem que ser duas linguagens diferentes

## 2 Apps standalone
faazem polling no mysql e geram as palavras nos eventos restantes

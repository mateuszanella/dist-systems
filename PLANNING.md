# Planning of the distributed system task



## Mysql

Rules: 
    - N pode ter PK, FK, indexes
    - n pode procedures, triggers, etc.
    - n há auto increment

## 2 Web Apps

Contratos:
GET /events
GET /events/{id}

POST /events
POST /events/async

- tem que ser duas linguagens diferentes

## 2 Apps standalone
faazem polling no mysql e geram as palavras nos eventos restantes

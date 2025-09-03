use crate::database::pool::DbPool;
use crate::models::event::Event;
use sqlx::{MySql, Transaction};
use std::time::Duration;
use rand::Rng;

const WORDS: &[&str] = &[
    "casa", "carro", "arvore", "flor", "ceu", "terra", "agua", "fogo", "vento", "sol",
    "lua", "estrela", "nuvem", "chuva", "rio", "mar", "montanha", "cidade", "pessoa", "amor",
    "paz", "alegria", "trabalho", "escola", "computador", "telefone", "livro", "caneta", "papel", "mesa",
    "cadeira", "porta", "janela", "rua", "parque", "jardim", "cozinha", "quarto", "banheiro", "comida",
    "bebida", "fruta", "legume", "pao", "leite", "cafe", "cha", "acucar", "sal", "pimenta",
    "chocolate", "biscoito", "bolo", "pudim", "sopa", "carne", "peixe", "frango", "arroz", "feijao",
    "macarrao", "pizza", "hamburguer", "sanduiche", "salada", "suco", "refrigerante", "cerveja", "vinho", "musica",
    "filme", "jogo", "esporte", "viagem", "ferias", "praia", "montanha", "floresta", "deserto", "neve",
    "gelo", "calor", "frio", "luz", "sombra", "barulho", "silencio", "tempo", "hora", "dia",
    "noite", "semana", "mes", "ano", "ontem", "hoje", "amanha", "sempre", "nunca", "muito",
    "pouco", "grande", "pequeno", "alto", "baixo", "forte", "fraco", "rapido", "devagar", "novo",
    "velho", "bonito", "feio", "bom", "ruim", "feliz", "triste", "rico", "pobre", "limpo",
    "sujo", "aberto", "fechado", "cheio", "vazio", "quente", "frio", "claro", "escuro", "direita",
    "esquerda", "frente", "atras", "emcima", "embaixo", "dentro", "fora", "perto", "longe", "verdade",
    "mentira", "pergunta", "resposta", "ajuda", "obrigado", "desculpe", "porfavor", "sim", "nao", "ola",
    "adeus"
];

pub async fn get_new_id(tx: &mut Transaction<'_, MySql>) -> Result<i32, sqlx::Error> {
    let current_id: i32 = sqlx::query_scalar("SELECT id FROM status FOR UPDATE")
        .fetch_one(&mut **tx)
        .await?;

    let new_id = current_id + 1;
    sqlx::query("UPDATE status SET id = ?")
        .bind(new_id)
        .execute(&mut **tx)
        .await?;

    Ok(new_id)
}

pub async fn create_sync_event(pool: &DbPool) -> Result<Event, Box<dyn std::error::Error>> {
    let event = create_async_event(pool).await?;

    // Poll for the event to be processed
    let timeout = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    loop {
        if start_time.elapsed() > timeout {
            return Err("Timeout waiting for event to be processed".into());
        }

        if let Some(processed_event) = get_event_by_id(pool, event.id).await? {
            if processed_event.value.is_some() {
                return Ok(processed_event);
            }
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

pub async fn create_async_event(pool: &DbPool) -> Result<Event, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let id = get_new_id(&mut tx).await?;

    sqlx::query("INSERT INTO events (id, value) VALUES (?, NULL)")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(Event::new(id, None))
}

pub async fn get_event_count(pool: &DbPool) -> Result<i32, sqlx::Error> {
    let count: i32 = sqlx::query_scalar("SELECT id FROM status")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

pub async fn get_event_by_id(pool: &DbPool, id: i32) -> Result<Option<Event>, sqlx::Error> {
    let event = sqlx::query_as::<_, Event>("SELECT id, value FROM events WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(event)
}

pub async fn process_event(pool: &DbPool) -> Result<bool, Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;

    let id: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM events WHERE value IS NULL LIMIT 1 FOR UPDATE SKIP LOCKED"
    )
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(event_id) = id {
        let value = generate_random_word();

        sqlx::query("UPDATE events SET value = ? WHERE id = ?")
            .bind(&value)
            .bind(event_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        tokio::time::sleep(Duration::from_millis(100)).await;

        log::info!("Processed event ID: {}", event_id);
        Ok(true)
    } else {
        drop(tx);
        Ok(false)
    }
}

fn generate_random_word() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..WORDS.len());
    WORDS[index].to_string()
}

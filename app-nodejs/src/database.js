const mysql = require('mysql2/promise');
const fs = require('fs').promises;
const path = require('path');

let pool;
let portugueseWords = [];

async function initDB() {
    const dbConfig = {
        host: 'mysql',
        user: 'root',
        password: '123123123',
        database: 'prod',
        waitForConnections: true,
        connectionLimit: 10,
        queueLimit: 0
    };

    pool = mysql.createPool(dbConfig);

    // Test connection and retry
    for (let i = 0; i < 5; i++) {
        try {
            await pool.getConnection();
            console.log('Database connection successful');
            break;
        } catch (err) {
            console.error(`Database ping failed, retrying in 5 seconds: ${err.message}`);
            await new Promise(resolve => setTimeout(resolve, 5000));
        }
    }

    try {
        await createSchema();
        console.log('Database schema verified');
    } catch (err) {
        console.error(`Could not create schema: ${err.message}`);
        process.exit(1);
    }

    try {
        await loadPortugueseWords();
        console.log(`Loaded ${portugueseWords.length} portuguese words`);
    } catch (err) {
        console.error(`Could not load portuguese words: ${err.message}`);
        process.exit(1);
    }
}

async function createSchema() {
    const connection = await pool.getConnection();
    try {
        await connection.execute(`
            CREATE TABLE IF NOT EXISTS events (
                id INT NOT NULL,
                value VARCHAR(255) NULL
            );
        `);
        await connection.execute(`
            CREATE TABLE IF NOT EXISTS status (
                id INT NOT NULL
            );
        `);

        const [rows] = await connection.execute('SELECT COUNT(*) as count FROM status');
        if (rows[0].count === 0) {
            await connection.execute('INSERT INTO status (id) VALUES (0)');
        }
    } finally {
        connection.release();
    }
}

async function getNewID(connection) {
    await connection.query('START TRANSACTION');
    try {
        const [rows] = await connection.execute('SELECT id FROM status FOR UPDATE');
        const currentID = rows[0].id;
        const newID = currentID + 1;
        await connection.execute('UPDATE status SET id = ?', [newID]);
        await connection.query('COMMIT');
        return newID;
    } catch (error) {
        await connection.query('ROLLBACK');
        throw error;
    }
}

function getRandomPortugueseWord() {
    if (portugueseWords.length === 0) {
        return "fallback_word";
    }
    const randomIndex = Math.floor(Math.random() * portugueseWords.length);
    return portugueseWords[randomIndex];
}

async function loadPortugueseWords() {
    const filePath = path.join(__dirname, '..', 'data', 'words.txt');
    const data = await fs.readFile(filePath, 'utf8');
    portugueseWords = data.split('\n').map(word => word.trim()).filter(word => word !== '');
    if (portugueseWords.length === 0) {
        throw new Error('No words found in words.txt');
    }
}

async function createSyncEvent() {
    const connection = await pool.getConnection();
    try {
        const newID = await getNewID(connection);
        await connection.execute('INSERT INTO events (id, value) VALUES (?, NULL)', [newID]);
        
        // Poll for the event to be processed
        const startTime = Date.now();
        const timeout = 30 * 1000; // 30 seconds
        const pollInterval = 200; // 200ms

        while (Date.now() - startTime < timeout) {
            const [rows] = await connection.execute('SELECT value FROM events WHERE id = ?', [newID]);
            if (rows.length > 0 && rows[0].value !== null) {
                return { id: newID, value: rows[0].value };
            }
            await new Promise(resolve => setTimeout(resolve, pollInterval));
        }
        throw new Error(`Timeout waiting for event ${newID} to be processed`);
    } finally {
        connection.release();
    }
}

async function createAsyncEvent() {
    const connection = await pool.getConnection();
    try {
        const newID = await getNewID(connection);
        await connection.execute('INSERT INTO events (id, value) VALUES (?, NULL)', [newID]);
        return { id: newID };
    } finally {
        connection.release();
    }
}

async function getEventCount() {
    const [rows] = await pool.execute('SELECT id FROM status');
    return rows[0].id;
}

async function getEventByID(id) {
    const [rows] = await pool.execute('SELECT id, value FROM events WHERE id = ?', [id]);
    if (rows.length === 0) {
        return null;
    }
    return { id: rows[0].id, value: rows[0].value };
}

async function processEvent() {
    const connection = await pool.getConnection();
    try {
        await connection.query('START TRANSACTION');
        const [rows] = await connection.execute('SELECT id FROM events WHERE value IS NULL LIMIT 1 FOR UPDATE SKIP LOCKED');

        if (rows.length === 0) {
            await connection.execute('ROLLBACK');
            return false; // No events to process
        }

        const eventID = rows[0].id;

        // Simulate work
        await new Promise(resolve => setTimeout(resolve, 100)); // 100ms

        const randomWord = getRandomPortugueseWord();
        await connection.execute('UPDATE events SET value = ? WHERE id = ?', [randomWord, eventID]);
        await connection.query('COMMIT');
        console.log(`Processed event ID: ${eventID}`);
        return true;
    } catch (error) {
        await connection.query('ROLLBACK');
        throw error;
    } finally {
        connection.release();
    }
}

module.exports = {
    initDB,
    createSyncEvent,
    createAsyncEvent,
    getEventCount,
    getEventByID,
    processEvent
};

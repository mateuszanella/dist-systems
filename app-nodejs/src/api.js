const express = require('express');
const database = require('./database');

const app = express();
app.use(express.json());

app.post('/events', async (req, res) => {
    try {
        const event = await database.createSyncEvent();
        res.status(201).json(event);
    } catch (error) {
        console.error('Error creating sync event:', error);
        res.status(500).json({ error: error.message });
    }
});

app.post('/events/async', async (req, res) => {
    try {
        const event = await database.createAsyncEvent();
        res.status(202).json(event);
    } catch (error) {
        console.error('Error creating async event:', error);
        res.status(500).json({ error: error.message });
    }
});

app.get('/events', async (req, res) => {
    try {
        const count = await database.getEventCount();
        res.status(200).json({ count });
    } catch (error) {
        console.error('Error getting event count:', error);
        res.status(500).json({ error: error.message });
    }
});

app.get('/events/:id', async (req, res) => {
    try {
        const id = parseInt(req.params.id);
        if (isNaN(id)) {
            return res.status(400).json({ error: 'Invalid ID' });
        }
        const event = await database.getEventByID(id);
        if (!event) {
            return res.status(404).json({ error: 'Event not found' });
        }
        res.status(200).json(event);
    } catch (error) {
        console.error('Error getting event by ID:', error);
        res.status(500).json({ error: error.message });
    }
});

const PORT = process.env.PORT || 8080;

async function startServer() {
    await database.initDB();
    app.listen(PORT, () => {
        console.log(`Node.js API server listening on port ${PORT}`);
    });
}

startServer();

const database = require('./database');

async function startWorker() {
    await database.initDB();
    console.log('Node.js worker started');

    while (true) {
        try {
            const processed = await database.processEvent();
            if (!processed) {
                // Wait before polling again if no event was processed
                await new Promise(resolve => setTimeout(resolve, 100)); // 100ms
            }
        } catch (error) {
            console.error('Error processing event in worker:', error);
            // Implement a backoff strategy here if needed
            await new Promise(resolve => setTimeout(resolve, 1000)); // Wait longer on error
        }
    }
}

startWorker();

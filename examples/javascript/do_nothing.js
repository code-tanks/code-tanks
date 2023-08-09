const { BaseTank, Command } = require('javascript-api');

class Tank extends BaseTank {
    constructor() {
        super();  // Call the parent constructor
        // Additional properties can be added here
    }

    run() {
        // Implement the parent's abstract run method
        console.log('Tank is running2');
        this.commands.push(Command.MOVE_FORWARD);
    }

    onEvent(event) {
        // Implement the parent's abstract onEvent method
        console.log(`Tank received event: ${event}`);
    }
}

function createTank() {
    return new Tank();
}

module.exports = {
    createTank
};
let username = '';

const ws = new WebSocket("ws://127.0.0.1:3030/chat");

ws.onmessage = function(event) {
    const messages = document.getElementById('messages');
    const messageElement = document.createElement('li');
    messageElement.innerText = event.data;
    messages.appendChild(messageElement);
    messages.scrollTop = messages.scrollHeight; // auto scroll to the latest message
};

document.getElementById('start-chat').addEventListener('click', function() {
    username = document.getElementById('splash-username').value;
    if (username.trim() !== '') {
        // Hide splash screen
        document.getElementById('splash-screen').style.display = 'none';
        // Show chat container
        document.getElementById('chat-container').classList.remove('hidden');
    }
});

function sendMessage() {
    const message = document.getElementById('message').value;
    if (message.trim() !== '') {
        const chatMessage = { user: username, message: message };
        ws.send(JSON.stringify(chatMessage));
        document.getElementById('message').value = ''; // clear the input
    }
}
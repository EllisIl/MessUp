let senderName = ""; // Variable to store sender name

document.getElementById('joinButton').addEventListener('click', () => {
    const nameInput = document.getElementById('nameInput');
    senderName = nameInput.value.trim();

    if (senderName) {
        document.getElementById('nameContainer').style.display = 'none'; // Hide name input
        document.getElementById('messagingContainer').style.display = 'block'; // Show messaging area

        // Enable message input and send button
        document.getElementById('messageInput').disabled = false;
        document.querySelector('button[type="submit"]').disabled = false;

        loadMessages(); // Load messages when entering the chat
        setInterval(loadMessages, 2000); // Set polling for messages
    } else {
        alert("Please enter a valid name."); // Alert for empty name
    }
});

document.getElementById('inputContainer').addEventListener('submit', async (event) => {
    event.preventDefault();
    const messageInput = document.getElementById('messageInput');
    const message = { sender: senderName, content: messageInput.value };

    // Send the message to the server
    await fetch('/send', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(message),
    });

    // Immediately display the sent message on the page
    addMessageToList(message);
    messageInput.value = ''; // Clear the input field
});

async function loadMessages() {
    const response = await fetch('/messages');
    const messages = await response.json();
    const messagesList = document.getElementById('messagesList');
    messagesList.innerHTML = '';

    messages.forEach(msg => {
        addMessageToList(msg);
    });

    // Automatically scroll to the bottom after loading messages
    const messagesContainer = document.getElementById('messagesContainer');
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
}

// Helper function to add a message to the list
function addMessageToList(msg) {
    const messagesList = document.getElementById('messagesList');
    const li = document.createElement('li');
    li.innerHTML = `<span class="message-sender">${msg.sender}:</span> ${msg.content}`; // Display sender and message
    messagesList.appendChild(li); // Append new message at the end
}

// Send message on button click
document.querySelector('button[type="submit"]').addEventListener('click', async () => {
    const messageInput = document.getElementById('messageInput');
    if (messageInput.value.trim()) {
        await sendMessage();
    }
});

// Send message function
async function sendMessage() {
    const messageInput = document.getElementById('messageInput');
    const message = { sender: senderName, content: messageInput.value };

    // Send the message to the server
    await fetch('/send', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(message),
    });

    // Immediately display the sent message on the page
    addMessageToList(message);
    messageInput.value = ''; // Clear the input field
}

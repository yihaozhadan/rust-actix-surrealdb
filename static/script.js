// Load todos when the page loads
document.addEventListener('DOMContentLoaded', loadTodos);

async function loadTodos() {
    try {
        const response = await fetch('/todos');
        const todos = await response.json();
        displayTodos(todos);
    } catch (error) {
        console.error('Error loading todos:', error);
    }
}

function displayTodos(todos) {
    const todoList = document.getElementById('todo-list');
    todoList.innerHTML = '';
    
    todos.forEach(todo => {
        // Extract the ID from the Thing object
        const todoId = todo.id?.id?.String || '';
        
        const todoItem = document.createElement('div');
        todoItem.className = `todo-item ${todo.completed ? 'completed' : ''}`;
        
        todoItem.innerHTML = `
            <span class="todo-text">${todo.description}</span>
            <div class="todo-actions">
                <button class="toggle" onclick="toggleTodo('${todoId}')">${todo.completed ? 'Undo' : 'Complete'}</button>
                <button class="delete" onclick="deleteTodo('${todoId}')">Delete</button>
            </div>
        `;
        
        todoList.appendChild(todoItem);
    });
}

async function addTodo() {
    const input = document.getElementById('new-todo');
    const description = input.value.trim();
    
    if (!description) return;
    
    try {
        const response = await fetch('/todo', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: `description=${encodeURIComponent(description)}`
        });
        
        if (response.ok) {
            input.value = '';
            loadTodos();
        }
    } catch (error) {
        console.error('Error adding todo:', error);
    }
}

async function toggleTodo(id) {
    try {
        const response = await fetch(`/todo/${id}`, {
            method: 'PUT'
        });
        
        if (response.ok) {
            loadTodos();
        }
    } catch (error) {
        console.error('Error toggling todo:', error);
    }
}

async function deleteTodo(id) {
    try {
        const response = await fetch(`/todo/${id}`, {
            method: 'DELETE'
        });
        
        if (response.ok) {
            loadTodos();
        }
    } catch (error) {
        console.error('Error deleting todo:', error);
    }
}

// Add keyboard event listener for the input field
document.getElementById('new-todo').addEventListener('keypress', function(e) {
    if (e.key === 'Enter') {
        addTodo();
    }
});

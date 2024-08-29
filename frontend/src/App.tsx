import React, { useEffect, useState } from "react";
import "./App.css";
import TodoItem from "./TodoItem.tsx";


function App() {
  const [todos, setTodos] = useState([]);
  const [content, setContent] = useState('');


  function getTodos() {
    fetch("http://localhost:8000/todos", {
      method: 'GET'
    })
      .then((res) => res.json())
      .then((res) => setTodos(res.todos))
      .catch(console.error);
  }

  useEffect(getTodos, [])

  function addTodo(event) {
    event.preventDefault()

    fetch("http://localhost:8000/todos", {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ content: content })
    }).then(getTodos)
      .catch(console.error);
  }

  return (<div>
    <header>
      <div className="centered header-display">
        <h1> Fancy TODO App</h1>
        <form onSubmit={addTodo}>
          <input name="content" type="text" value={content} onChange={(e) => setContent(e.target.value)} ></input> <button type="submit"> Hinzufügen</button>
        </form>
      </div>
    </header>
    <div className="container">
      <div className="centered">

        <ul>
          {todos.map(todo =>
            <TodoItem item={todo} refreshTodos={getTodos}></TodoItem>
          )}
        </ul>
      </div></div></div>
  );
}

export default App;
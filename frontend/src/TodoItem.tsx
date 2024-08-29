import React from "react";
import "./TodoItem.css";

export interface TodoItem {
  id: number;
  content: string
}

function TodoItem(input: { item: TodoItem, refreshTodos: () => void }) {
  function click() {
    fetch(`http://localhost:8000/todos/${input.item.id}`, {
      method: 'DELETE',
    }).then(input.refreshTodos)
      .catch(console.error);
  }

  return (
    <li key={input.item.id}> <p>{input.item.content}</p> <button  className='button' onClick={click}>Löschen</button>
    </li>
  );
}

export default TodoItem;
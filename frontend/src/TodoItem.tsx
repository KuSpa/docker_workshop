import React from "react";

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
    <li> {input.item.content} <button onClick={click}>Löschen</button>
    </li>
  );
}

export default TodoItem;
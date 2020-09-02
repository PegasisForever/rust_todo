import React from "react"
import Login from "./Login"
import {postData, removeItemOnce} from "./tools"
import {API_BASE_PATH} from "./App"

export default class List extends React.Component {
    constructor(props) {
        super(props)

        this.getTodoList = this.getTodoList.bind(this)
        this.toggleTodo = this.toggleTodo.bind(this)
        this.addTodo = this.addTodo.bind(this)
        this.removeTodo = this.removeTodo.bind(this)
        this.getTodoLi = this.getTodoLi.bind(this)

        this.state = {
            todos: [],
            newTodoName: "",
        }
    }

    componentDidMount() {
        this.getTodoList()
    }

    getTodoLi(todoList) {
        return todoList.map((todo) =>
            <li key={todo.id}>
                {todo.completed ?
                    <del>{todo.name}</del> :
                    <span>{todo.name}</span>}
                {todo.completed ?
                    <button onClick={() => this.toggleTodo(todo, false)}>Restore</button> :
                    <button onClick={() => this.toggleTodo(todo, true)}>Complete</button>}
                <button onClick={() => this.removeTodo(todo)}>Remove</button>
            </li>)
    }

    render() {
        let completed = []
        let notCompleted = []
        this.state.todos.slice().reverse().forEach((todo) => {
            if (todo.completed) {
                completed.push(todo)
            } else {
                notCompleted.push(todo)
            }
        })

        return <div>
            <button onClick={() => this.props.changePage(<Login changePage={this.props.changePage}/>)}>
                Logout
            </button>
            <button onClick={this.getTodoList}>
                Refresh
            </button>
            <form onSubmit={(e) => {
                e.preventDefault()
                if (this.state.newTodoName === "") {
                    alert("Todo name cannot be empty!")
                } else {
                    this.addTodo(this.state.newTodoName)
                }
            }}>
                <label>
                    Add todo:
                    <input type="text" name="add"
                           value={this.state.newTodoName}
                           onChange={(e) => this.setState({
                               newTodoName: e.target.value,
                           })}/>
                </label>
                <input type="submit" value="Add"/>
            </form>
            <ul>
                {this.getTodoLi(notCompleted)}
                {this.getTodoLi(completed)}
            </ul>
        </div>
    }

    getTodoList() {
        postData(
            API_BASE_PATH + "list",
            {session_id: this.props.sessionId},
            (status, response) => {
                if (status === 200) {
                    this.setState({
                        todos: JSON.parse(response),
                    })
                } else if (status === 403) {
                    alert(`Please login.`)
                    this.props.changePage(<Login changePage={this.props.changePage}/>)
                } else {
                    alert(`Unknown error: ${status}`)
                }
            },
        )
    }

    toggleTodo(todo, completed) {
        postData(
            API_BASE_PATH + "toggle",
            {
                session_id: this.props.sessionId,
                todo_id: todo.id,
                completed: completed,
            },
            (status, _) => {
                if (status === 200) {
                    todo.completed = completed
                    this.setState({})
                } else if (status === 403) {
                    alert(`Please login.`)
                    this.props.changePage(<Login changePage={this.props.changePage}/>)
                } else {
                    alert(`Unknown error: ${status}`)
                }
            },
        )
    }

    addTodo(name) {
        postData(
            API_BASE_PATH + "add",
            {
                session_id: this.props.sessionId,
                todo_name: name,
            },
            (status, response) => {
                if (status === 200) {
                    this.state.todos.push({
                        id: JSON.parse(response)["todo_item_id"],
                        name: name,
                        completed: false,
                    })
                    this.setState({
                        newTodoName: "",
                    })
                } else if (status === 403) {
                    alert(`Please login.`)
                    this.props.changePage(<Login changePage={this.props.changePage}/>)
                } else {
                    alert(`Unknown error: ${status}`)
                }
            },
        )
    }

    removeTodo(todo) {
        postData(
            API_BASE_PATH + "remove",
            {
                session_id: this.props.sessionId,
                todo_id: todo.id,
            },
            (status, response) => {
                if (status === 200) {
                    removeItemOnce(this.state.todos, todo)
                    this.setState({})
                } else if (status === 403) {
                    alert(`Please login.`)
                    this.props.changePage(<Login changePage={this.props.changePage}/>)
                } else {
                    alert(`Unknown error: ${status}`)
                }
            },
        )
    }
}

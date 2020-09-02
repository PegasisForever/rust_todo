import React from "react"
import Login from "./Login"
import postData from "./tools"
import {API_BASE_PATH} from "./App"

export default class List extends React.Component {
    constructor(props) {
        super(props)

        this.getTodoList = this.getTodoList.bind(this)
        this.toggleTodo = this.toggleTodo.bind(this)
        this.addTodo = this.addTodo.bind(this)

        this.state = {
            todos: [],
            newTodoName: "",
        }
    }

    componentDidMount() {
        this.getTodoList()
    }

    render() {
        return <div>
            <button onClick={() => this.props.changePage(<Login changePage={this.props.changePage}/>)}>
                Logout
            </button>
            <button onClick={this.getTodoList}>
                Refresh
            </button>
            <form onSubmit={(e) => {
                console.log("awa")
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
                {this.state.todos.map((todo) =>
                    <li key={todo.id}>
                        {todo.completed ?
                            <del>{todo.name}</del> :
                            <span>{todo.name}</span>}
                        {todo.completed ?
                            <button onClick={() => this.toggleTodo(todo, false)}>Restore</button> :
                            <button onClick={() => this.toggleTodo(todo, true)}>Complete</button>}
                        <button>Delete</button>
                    </li>)}
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
                        newTodoText: "",
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
}

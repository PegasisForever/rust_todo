import React from "react"
import postData from "./tools"
import {API_BASE_PATH} from "./App"
import List from "./List"
import Regi from "./Regi"

export default class Login extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            name: "",
            password: "",
        }
    }

    render() {
        return <div>
            <button onClick={() => this.props.changePage(<Regi changePage={this.props.changePage}/>)}>
                Register
            </button>
            <h2>Login</h2>
            <form onSubmit={async (e) => {
                e.preventDefault()
                postData(API_BASE_PATH + "login", this.state, (status, response) => {
                    if (status === 200) {
                        this.props.changePage(<List
                            changePage={this.props.changePage}
                            sessionId={JSON.parse(response)["session_id"]}/>)
                    } else if (status === 403) {
                        alert("Username or password incorrect.")
                    } else {
                        alert(`Unknown error: ${status}`)
                    }
                })
            }}>
                <label>
                    Name:
                    <input type="text" name="name"
                           value={this.state.name}
                           onChange={(e) => this.setState({
                               name: e.target.value,
                           })}/>
                </label>
                <br/>
                <label>
                    Password:
                    <input type="password" name="password"
                           value={this.state.password}
                           onChange={(e) => this.setState({
                               password: e.target.value,
                           })}/>
                </label>
                <br/>
                <input type="submit" value="Login"/>
            </form>
        </div>
    }
}

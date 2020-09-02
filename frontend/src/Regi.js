import React from "react"
import postData from "./tools"
import {API_BASE_PATH} from "./App"
import Login from "./Login"

export default class Regi extends React.Component {
    constructor(props) {
        super(props)
        this.state = {
            name: "",
            password: "",
        }
    }

    render() {
        return <div>
            <button onClick={() => this.props.changePage(<Login changePage={this.props.changePage}/>)}>
                Login
            </button>
            <h2>Register</h2>
            <form onSubmit={async (e) => {
                e.preventDefault()
                postData(API_BASE_PATH + "regi", this.state, (status, _) => {
                    if (status === 200) {
                        alert("Success!")
                    } else if (status === 409) {
                        alert("This use already exists.")
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
                <input type="submit" value="Register"/>
            </form>
        </div>
    }
}

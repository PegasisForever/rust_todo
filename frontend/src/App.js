import React from "react"
import Login from "./Login"

export const API_BASE_PATH = "http://localhost:8001/"

export default class App extends React.Component {
    constructor(props) {
        super(props)

        this.changePage = this.changePage.bind(this)

        this.state = {
            page: <Login changePage={this.changePage} setSessionId={this.setSessionId}/>,
        }
    }

    render() {
        return <div>
            <h1>Rust Todo</h1>
            {this.state.page}
        </div>
    }

    changePage(newPage){
        this.setState({
            page:newPage,
        })
    }
}

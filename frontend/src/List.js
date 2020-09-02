import React from "react"

export default class List extends React.Component {
    render() {
        return <div>
            <button>Logout</button>
            <h2>{this.props.sessionId}</h2>
        </div>
    }
}

import logo from './logo.svg';
import './App.css';
import React from 'react';

function Toggle(props){

    return (
      <button className="button-primary" 
        onClick={() => props.onClick()}
        style={props.style}>
        {props.value} 
      </button>
    )
  
}

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      buttonOn: false,
    }
  }

  handleClick() 
  {
    this.setState({buttonOn: !this.state.buttonOn});
  } 

  newToggle()
  {
    return (
      <Toggle 
        onClick={() => this.handleClick()} 
        value={this.state.buttonOn ? 'On' : 'Off'} 
        style={this.state.buttonOn ? {background: 'green'} : {background: 'red'}} 
      />
    )
  }


  render() {
    
    return (
      <div className="App">
        <header className="App-header">
          <div>
            <h1>Online Services, LTD</h1>
            <p>Welcome to Online Services, LTD. We provide the ressources you and your team need, at a low price. We offer flexible solutions for your small business needs. We offer web development, logo creation, video editing and more. Feel free to get in contact today and kickstart your company's online success.</p>
            <p>Press the button below to become a better business.</p>
            {this.newToggle()}
            <p>{this.state.buttonOn ? 'You now manage a better business.' : 'Thanks for reading. Do consider toggling the button above.'}</p>

            <p className="mini">Note: it's not that easy to become a better business. You must contact us to get a free quote, then spend lots of money on our services.</p>
          </div>
        </header>
      </div>
    );
  }
}

export default App;

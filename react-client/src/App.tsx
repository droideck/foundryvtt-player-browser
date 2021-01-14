import React, { useState } from 'react';
import './App.css';

function App() {
    import('wasm').then(({ print_string }) => {
    const strResult = print_string();
    //const strResult = "World";
    setStr(strResult);
  })
  const [strFile, setStr] = useState("");

  return (
    <div className="App" >
      <div>String Results: {strFile}</div>
    </div>
  );
}

export default App;

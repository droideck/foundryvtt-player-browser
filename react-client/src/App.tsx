import React, { useState } from 'react';
import { Tabs, Badge } from 'antd-mobile';
import './App.css';


function App() {
    import('wasm').then(({ print_string }) => {
    const strResult = print_string("Hello");
    //const strResult = "World";
    setStr(strResult);
  })
  const [strFile, setStr] = useState("");
  const tabs = [
    { title: <Badge text={'13'}>First Tab</Badge> },
    { title: <Badge text={'® Test'}>Second Tab</Badge> },
    { title: <Badge dot>Third Tab</Badge> },
  ];

  const TabExample = () => (
    <div>
      <Tabs tabs={tabs}
        initialPage={1}
        onChange={(tab, index) => { console.log('onChange', index, tab); }}
        onTabClick={(tab, index) => { console.log('onTabClick', index, tab); }}
      >
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '200px', backgroundColor: '#fff' }}>
          {strFile} - Content of first tab
        </div>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '200px', backgroundColor: '#fff' }}>
          Content of second tab
        </div>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '200px', backgroundColor: '#fff' }}>
          Content of third tab
        </div>
      </Tabs>
    </div>
  );

  return (
    <div className="App" >
        <TabExample />
    </div>
  );
}

export default App;

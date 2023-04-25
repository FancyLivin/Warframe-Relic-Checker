import lithRelic from './Images/LithRelicIntact.webp';
import MesoRelic from './Images/MesoRelicIntact.webp';
import NeoRelic from './Images/NeoRelicIntact.webp';
import AxiRelic from './Images/AxiRelicIntact.webp';

import './App.css';
import SearchBar from './Components/searchBar';
import RelicData from './Data.json'

function App() {
  return (
    
    <div className="App">
          <SearchBar placeholder={'Search...'}/>
    </div>

  );
}

export default App;

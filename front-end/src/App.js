import './App.css';
import SearchBar from './Components/searchBar';
import RelicDisplay from './Components/RelicDisplay';

function App() {
  return (
    <div className="App">
          <SearchBar placeholder={'Search...'}/>
          <RelicDisplay />
    </div>

  );
}

export default App;

import React from 'react'
import './searchBar.css'
import SearchIcon from '@mui/icons-material/Search';

function SearchBar({placeholder, data}) {
    return (
        <div className='search'>
            <div className='searchInputs'>
                <input type='text' placeholder={placeholder}/>
                <div className='searchIcon'>
                    <SearchIcon />
                </div>
            </div>
            <div className='dataResult'></div>
        </div>
    );
}

export default SearchBar;
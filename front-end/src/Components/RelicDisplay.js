import lithRelic from '../Images/LithRelicIntact.webp';
import MesoRelic from '../Images/MesoRelicIntact.webp';
import NeoRelic from '../Images/NeoRelicIntact.webp';
import AxiRelic from '../Images/AxiRelicIntact.webp';

import './RelicDisplay.css';
import RelicData from '../Data.json'

function RelicDisplay() {

    return(
        <div>
            <img className='RelicView'
                src={lithRelic}
            />
            <img className='RelicView'
                src={MesoRelic}
            />
            <br/>
            <img className='RelicView'
                src={NeoRelic}
            />
            <img className='RelicView'
                src={AxiRelic}
            />


            <div className='Test'>    
                {
                    RelicData && RelicData.map( primeItem => {
                        return (
                            <div className='Rectangle' key = {primeItem.name}>
                                *{ primeItem.name }

                                { 
                                    primeItem.components && primeItem.components.map( primeComponent => {
                                        return(
                                            <div key = {primeItem.name}>
                                                + { primeComponent.name }

                                            {
                                                primeComponent.relics && primeComponent.relics.map( primeRelics => {
                                                    return(
                                                        <div key = {primeItem.name}>
                                                            -- { primeRelics.name }
                                                        </div>
                                                    )
                                                })
                                            }
                                            </div>
                                        )
                                    })
                                }
                            </div>
                        )
                    })
                }
            </div>
        </div>
    );
}

export default RelicDisplay;
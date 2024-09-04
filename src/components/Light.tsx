type LightState = "on" | "off";

interface LightProps {
    state: LightState;
    turnOn: () => void;
    turnOff: () => void;
}

import React, { useState } from 'react';

const Light: React.FC<LightProps> = ({ state, turnOn, turnOff }) => {
    return (
        <div>
            <p>The light is {state}</p>
            {state === 'off' && <button onClick={turnOn}>Turn On</button>}
            {state === 'on' && <button onClick={turnOff}>Turn Off</button>}
        </div>
    );
};

const LightContainer: React.FC = () => {
    const [state, setState] = useState<LightState>('off');

    const turnOn = () => setState('on');
    const turnOff = () => setState('off');

    return <Light state={state} turnOn={turnOn} turnOff={turnOff} />;
};

export default LightContainer;
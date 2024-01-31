import React from 'react';

interface ButtonTemplate
{
    title: string;
    disabled: boolean;
}

function ButtonMaxxing({ title, disabled }: ButtonTemplate)
{
    return <button disabled={disabled} >{title}</button>;
}

export default ButtonMaxxing
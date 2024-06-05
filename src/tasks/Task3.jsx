import { useState, useEffect } from "react";
import { useInputHandler, useNumArgHandler } from "../utils/handlers";

import { MxArgRender } from "../utils/renders";

import { invoke } from "@tauri-apps/api/tauri";
import OutputComponent from "../components/OutputComponent";
import InputBooleanFunction from "../components/InputBooleanFunction";

const Task3 = () => {

    const left = useInputHandler({mn:0});
    const right = useInputHandler({mn:0});

    const {
        numArg, handleArgsInput
    } = useNumArgHandler();

    const [rightOk, setRightOk] = useState(false);
    const [output, setOutput] = useState("-");
    const [err, setErr] = useState("");

    useEffect(()=>{
        if( left.isOk && right.inputValue.length === left.inputValue.length ) {
            setRightOk(true);
        } else {
            setRightOk(false);
        }
    }, [right.inputValue, left.inputValue]);

    useEffect(() => {
        
        if(left.isOk && right.isOk && numArg.length !== 0 && right.inputValue.length === left.inputValue.length) {
            invoke( "get_func_from_remainde", { func0: left.inputValue, func1: right.inputValue, n: numArg } )
            .then((message) => {
                setErr("");
                setOutput(message);
            }).catch((err) => {
                setErr(err);
            });
        }
        if(numArg.length === 0 || left.inputValue.length === 0 || right.inputValue.length === 0) {
            setErr("");
            setOutput("-");
        }
    });

    return (
        <>
            <div className="grid grid-cols-2 grid-rows-1 gap-4 select-none">
                
                <div className="">
                    <InputBooleanFunction
                        isOk={left.isOk} inputValue={left.inputValue}
                        handleInput={left.handleInput}
                        placeholder={"Нулувая остаточная"}
                        maxLength={16}
                    />
                </div>
                <div className="">
                    <InputBooleanFunction
                        placeholder={"Единичная остаточная"}
                        maxLength={16}
                        inputValue={right.inputValue}
                        handleInput={right.handleInput}
                        isOk={rightOk}
                    />
                </div>
                <div className="">
                    <input placeholder="Номер аргумента" onInput={handleArgsInput} value={numArg} maxLength="1"></input>
                    <div className="px-3 py-1 text-slate-600"><MxArgRender value={left.inputValue.length > 0 ? left.mxArg + 1 : left.mxArg}/></div>
                </div>
                
            </div>
            <div className="pt-8 text-center">
                <div className="text-slate-600 text-sm select-none">Функция:</div>
                <div className="pt-1 text-xl break-words mx-24">
                    <OutputComponent err={err} output={output}/>
                </div>
            </div>
        </>
    )
}

export default Task3;
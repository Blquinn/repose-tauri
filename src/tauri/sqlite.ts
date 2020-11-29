import { promisified } from 'tauri/api/tauri';

export type Arg = string | number | boolean | null;

interface Stmt {
    stmt: string,
    args: Arg[],
}


type Exec = {"exec": Stmt}
type Select = {"select": Stmt}
type SelectOne = {"selectOne": Stmt}

type Commands =
    Exec
    | Select
    | SelectOne;


type ExecResponse = {num_rows_effected: number}
type SelectResponse = {rows: Arg[][]}
type SelectOneResponse = {row: Arg[]}

type ExecResponseWrapper = {"exec": ExecResponse}
type SelectResponseWrapper = {"select": SelectResponse}
type SelectOneResponseWrapper = {"selectOne": SelectOneResponse}

type Responses =
    ExecResponseWrapper
    | SelectResponseWrapper
    | SelectOneResponseWrapper;


export async function exec(stmt: string, ...args: Arg[]): Promise<ExecResponse> {
    const cmd: Commands = {"exec": {stmt, args}};

    return (await promisified<Responses>({
        cmd: 'sqliteCommand',
        command: cmd,
    }) as ExecResponseWrapper).exec;
}

export async function select(stmt: string, ...args: Arg[]): Promise<SelectResponse> {
    const cmd: Commands = {"select": {stmt, args}};

    return (await promisified<Responses>({
        cmd: 'sqliteCommand',
        command: cmd,
    }) as SelectResponseWrapper).select;
}

export async function selectOne(stmt: string, ...args: Arg[]): Promise<SelectOneResponse> {
    const cmd: Commands = {"selectOne": {stmt, args}};

    return (await promisified<Responses>({
        cmd: 'sqliteCommand',
        command: cmd,
    }) as SelectOneResponseWrapper).selectOne;
}

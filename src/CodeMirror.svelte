<script lang="ts">
    // TODO: Store undo/redo info in the request state.
    // TODO: Unify color scheme.

    import CodeMirror from './codemirror';
    import {onMount, createEventDispatcher} from 'svelte';
    import {editorModes} from "./consts";

    export let show: boolean = true;

    const dispatch = createEventDispatcher();
    export let readonly = false;
    export let errorLoc = null;
    export let flex = false;
    export let lineNumbers = true;
    export let tab = true;
    let w;
    let h;
    let code = '';
    let mode;

    // We have to expose set and update methods, rather
    // than making this state-driven through props,
    // because it's difficult to update an editor
    // without resetting scroll otherwise
    export async function set(new_code: string, new_mode: string) {
        console.log('Setting editor contents.')

        if (new_mode !== mode) {
            // createEditor(mode = new_mode);
            // editor.setMode(new_mode)
            let mode = editorModes[new_mode] || {name: new_mode}
            editor.setOption("mode", mode);
        }
        code = new_code;
        updating_externally = true;
        if (editor) {
            editor.setValue(code);
            setTimeout(function() {
                editor.refresh();
            }, 1);
        }
        updating_externally = false;
    }

    export function getValue(): string {
        return editor ? editor.getValue() : '';
    }

    export function update(new_code: string) {
        code = new_code;
        if (editor) {
            const {left, top} = editor.getScrollInfo();
            editor.setValue(code = new_code);
            editor.scrollTo(left, top);
        }
    }

    export function resize() {
        editor.refresh();
    }

    export function focus() {
        editor.focus();
    }

    export function getHistory() {
        return editor.getHistory();
    }

    export function setHistory(history) {
        editor.setHistory(history);
    }

    export function clearHistory() {
        if (editor) editor.clearHistory();
    }

    const refs: any = {};
    let editor;
    let updating_externally = false;
    let marker;
    let error_line;
    let destroyed = false;
    $: if (editor && w && h) {
        editor.refresh();
    }
    // $: {
    //     if (marker) marker.clear();
    //     if (errorLoc) {
    //         const line = errorLoc.line - 1;
    //         const ch = errorLoc.column;
    //         marker = editor.markText({line, ch}, {line, ch: ch + 1}, {
    //             className: 'error-loc'
    //         });
    //         error_line = line;
    //     } else {
    //         error_line = null;
    //     }
    // }
    // let previous_error_line;
    // $: if (editor) {
    //     if (previous_error_line != null) {
    //         editor.removeLineClass(previous_error_line, 'wrap', 'error-line')
    //     }
    //     if (error_line && (error_line !== previous_error_line)) {
    //         editor.addLineClass(error_line, 'wrap', 'error-line');
    //         previous_error_line = error_line;
    //     }
    // }
    onMount(() => {
        // createEditor(mode || 'plain');

        createEditor(mode || 'json');


        // (async () => {
        //     // if (!_CodeMirror) {
        //     //     let mod = await codemirror_promise;
        //     //     CodeMirror = mod.default;
        //     // } else {
        //     //     CodeMirror = _CodeMirror;
        //     // }
        //     await createEditor(mode || 'svelte');
        //     if (editor) editor.setValue(code || '');
        // })();
        // return () => {
        //     destroyed = true;
        //     if (editor) editor.toTextArea();
        // }
    });
    // let first = true;

    function createEditor(mode) {
        if (destroyed || !CodeMirror) return;
        // if (editor) editor.toTextArea();
        const opts = {
            lineNumbers,
            lineWrapping: true,
            indentWithTabs: true,
            indentUnit: 2,
            tabSize: 2,
            value: '',
            mode: editorModes[mode] || {
                name: mode
            },
            readOnly: readonly,
            autoCloseBrackets: true,
            autoCloseTags: true,
            extraKeys: {
                'Enter': 'newlineAndIndentContinueMarkdownList',
                'Ctrl-/': 'toggleComment',
                'Cmd-/': 'toggleComment',
                'Ctrl-Q': function (cm) {
                    cm.foldCode(cm.getCursor());
                },
                'Cmd-Q': function (cm) {
                    cm.foldCode(cm.getCursor());
                }
            },
            foldGutter: true,
            gutters: ['CodeMirror-linenumbers', 'CodeMirror-foldgutter']
        };
        if (!tab) {
            opts.extraKeys['Tab'] = tab;
            opts.extraKeys['Shift-Tab'] = tab;
        }
        // Creating a text editor is a lot of work, so we yield
        // the main thread for a moment. This helps reduce jank
        if (destroyed) return;
        editor = CodeMirror(refs.editor, opts);
        // editor = CodeMirror.fromTextArea(refs.editor, opts);
        editor.on('change', instance => {
            if (!updating_externally) {
                dispatch('change', null);
            }
        });
        editor.on('blur', instance => {
            if (!updating_externally) {
                dispatch('blur', null);
            }
        });
        editor.refresh();
    }
</script>

<style>
    .codemirror-container {
        flex: 1;
        position: relative;
        line-height: 1.5;
    }

    .editor {
        position: absolute;
        height: 100%;
        width: 100%;
    }

    .editor :global(.CodeMirror) {
        width: 100%;
        height: 100%;
        background: transparent;
        font: 400 14px/1.7 var(--font-mono);
        color: var(--base);
    }

    .editor.flex :global(.CodeMirror-lines) {
        padding: 0;
    }

    .editor :global(.CodeMirror-gutters) {
        padding: 0 16px 0 8px;
        border: none;
    }

    /*.editor :global(.error-loc) {*/
    /*    position: relative;*/
    /*    border-bottom: 2px solid #da106e;*/
    /*}*/

    /*.editor :global(.error-line) {*/
    /*    background-color: rgba(200, 0, 0, .05);*/
    /*}*/

    .codemirror-container.flex :global(.CodeMirror-lines) {
        padding: 0;
    }

    .codemirror-container :global(.CodeMirror-gutters) {
        padding: 0 16px 0 8px;
        border: none;
    }

    textarea {
        visibility: hidden;
    }
</style>

<div style="{show ? '' : 'display: none;'}"
    class="codemirror-container" class:flex bind:offsetWidth={w} bind:offsetHeight={h}>
    <!-- svelte-ignore a11y-positive-tabindex -->
<!--    <textarea-->
<!--            class="editor"-->
<!--            tabindex='2'-->
<!--            bind:this={refs.editor}-->
<!--            readonly-->
<!--            value={code}-->
<!--    ></textarea>-->
    <div class="editor" class:flex tabindex='2' bind:this={refs.editor}></div>
</div>

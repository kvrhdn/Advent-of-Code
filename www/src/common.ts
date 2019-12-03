export function setupDay(day: string, part1: Function, part2: Function) {
    const inputTextArea = document.getElementById(`input_d${day}`) as HTMLTextAreaElement;
    let responsePart1 = document.getElementById(`response_d${day}p1`) as HTMLSpanElement;
    let responsePart2 = document.getElementById(`response_d${day}p2`) as HTMLSpanElement;
    
    const solveButton = document.getElementById(`solve_d${day}`) as HTMLButtonElement;
    
    solveButton.onclick = (e: Event) => {
        const input = inputTextArea.value;
    
        try {
            const resultPart1 = withTimer('part1', () => part1(input));
            responsePart1.innerText = resultPart1.toString();
    
            const resultPart2 = withTimer('part2', () => part2(input));
            responsePart2.innerText = resultPart2.toString();
    
        } catch (err) {
            console.error(`An error occured: ${err}.`);
        }
    }
}

/**
 * Times the execution of func using console.time. Returns what func
 * has returned.
 */
function withTimer(name: string, func: Function): any {
    console.time(name);

    let result = func();

    console.timeEnd(name);
    return result;
}

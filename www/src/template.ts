/** 
 * Connect the HTML elements present in template.html with part1 and part2.
 */
export function setupTemplate(part1: Function, part2: Function) {
    const inputTextArea = document.getElementById(`input`) as HTMLTextAreaElement;
    let responsePart1 = document.getElementById(`response_part1`) as HTMLSpanElement;
    let responsePart2 = document.getElementById(`response_part2`) as HTMLSpanElement;

    const solveButton = document.getElementById(`solve_button`) as HTMLButtonElement;

    solveButton.onclick = (e: Event) => {
        const input = inputTextArea.value;

        try {
            const resultPart1 = withTimer('part1', () => part1(input));
            responsePart1.innerText = resultPart1.toString();
        } catch (err) {
            console.error(`An error occured running part 1: ${err}.`);
        }

        try {
            const resultPart2 = withTimer('part2', () => part2(input));
            responsePart2.innerText = resultPart2.toString();
        } catch (err) {
            console.error(`An error occured running part2: ${err}.`);
        }
    }
}

/**
 * Times the execution of func using console.time. Returns what func returned.
 */
function withTimer(name: string, func: Function): any {
    console.time(name);

    let result = func();

    console.timeEnd(name);
    return result;
}

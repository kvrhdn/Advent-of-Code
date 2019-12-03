import { part1, part2 } from '../../day03/pkg/day03';

const inputTextArea = document.getElementById('input_d03') as HTMLTextAreaElement;
let responsePart1 = document.getElementById('response_d03p1') as HTMLSpanElement;
let responsePart2 = document.getElementById('response_d03p2') as HTMLSpanElement;

const solveButton = document.getElementById('solve_d03') as HTMLButtonElement;

solveButton.onclick = (e: Event) => {
    const input = inputTextArea.value;

    try {
        const resultPart1 = part1(input);
        responsePart1.innerText = resultPart1.toString();

        const resultPart2 = part2(input);
        responsePart2.innerText = resultPart2.toString();
    } catch (err) {
        console.error(`An error occured: ${err}.`);
        alert(`An error occured: ${err}.`);
    }
};

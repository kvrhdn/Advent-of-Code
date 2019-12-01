import { part1, part2 } from '../../day01/pkg/day01';

const inputTextArea = document.getElementById('input_d01') as HTMLTextAreaElement;
let responsePart1 = document.getElementById('response_d01p1') as HTMLSpanElement;
let responsePart2 = document.getElementById('response_d01p2') as HTMLSpanElement;

const solveButton = document.getElementById('solve') as HTMLButtonElement;

solveButton.onclick = (e: Event) => {
    const input = inputTextArea.value;

    console.log(`Solving for input: ${input}`)

    const resultPart1 = part1(input);
    responsePart1.innerText = resultPart1;

    console.log(`Solved part 1: ${resultPart1}`);

    const resultPart2 = part2(input);
    responsePart2.innerText = resultPart2;

    console.log(`Solved part 2: ${resultPart2}`);
};

import { readFileSync } from "fs"

function part1() {
    const lines = readFileSync("input.txt", "utf8").toString().split("\n")
    const regexValueGoes = /value (?<value>\d+) goes to bot (?<bot>\d+)/
    const regexGivesLowHigh =
        /bot (?<senderBot>\d+) gives low to (?<lowReceiver>\w+) (?<lowValue>\d+) and high to (?<highReceiver>\w+) (?<highValue>\d+)/

    const botWithChips: Map<number, number[]> = new Map()
    const fromToLow: Map<number, [number, string]> = new Map()
    const fromToHigh: Map<number, [number, string]> = new Map()
    const outputs: Map<number, number> = new Map()

    for (const line of lines) {
        const groups = line.match(regexValueGoes)?.groups
        if (groups) {
            if (botWithChips.has(+groups.bot)) {
                const arr = botWithChips.get(+groups.bot)
                arr.push(+groups.value)
            } else {
                botWithChips.set(+groups.bot, [+groups.value])
            }
        } else {
            const groups = line.match(regexGivesLowHigh).groups
            fromToLow.set(+groups.senderBot, [
                +groups.lowValue,
                groups.lowReceiver,
            ])
            fromToHigh.set(+groups.senderBot, [
                +groups.highValue,
                groups.highReceiver,
            ])
        }
    }

    while (botWithChips.size > 0) {
        for (const [key, arr] of botWithChips.entries()) {
            if (arr.length >= 2) {
                const [elem1, elem2] = arr
                const [low, high] =
                    elem1 <= elem2 ? [elem1, elem2] : [elem2, elem1]

                if (low === 17 && high === 61) {
                    console.log(`Part 1 answer: ${key}`)
                }

                const [lowNum, lowType] = fromToLow.get(key)
                if (lowType === "bot") {
                    if (botWithChips.has(lowNum)) {
                        const arr = botWithChips.get(lowNum)
                        arr.push(low)
                    } else {
                        botWithChips.set(lowNum, [low])
                    }
                } else {
                    outputs.set(lowNum, low)
                }

                const [highNum, highType] = fromToHigh.get(key)
                if (highType === "bot") {
                    if (botWithChips.has(highNum)) {
                        const arr = botWithChips.get(highNum)
                        arr.push(high)
                    } else {
                        botWithChips.set(highNum, [high])
                    }
                } else {
                    outputs.set(highNum, high)
                }

                botWithChips.delete(key)
            }
        }
    }

    console.log(`botWithChips: ${[...botWithChips.entries()]}`)
    console.log(`fromToLow: ${[...fromToLow.entries()]}`)
    console.log(`fromToHigh: ${[...fromToHigh.entries()]}`)
    console.log(`outputs: ${[...outputs.entries()]}`)

    const part2Answer = outputs.get(0) * outputs.get(1) * outputs.get(2)
    console.log(`Part 2 answer: ${part2Answer}`)
}

part1()

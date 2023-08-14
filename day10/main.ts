import { readFileSync } from "fs"

function addToMapArray(map: Map<number, number[]>, key, value) {
    if (map.has(key)) {
        const arr = map.get(key)
        arr.push(value)
    } else {
        map.set(key, [value])
    }
}

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
            addToMapArray(botWithChips, +groups.bot, +groups.value)
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
            if (arr.length < 2) {
                continue
            }

            const [elem1, elem2] = arr
            const [low, high] = elem1 <= elem2 ? [elem1, elem2] : [elem2, elem1]

            if (low === 17 && high === 61) {
                console.log(`Part 1 answer: ${key}`)
            }

            const forwardValue = (
                fromTo: Map<number, [number, string]>,
                key: number,
                val: number
            ) => {
                const [num, type] = fromTo.get(key)
                if (type === "bot") {
                    addToMapArray(botWithChips, num, val)
                } else {
                    outputs.set(num, val)
                }
            }

            forwardValue(fromToLow, key, low)
            forwardValue(fromToHigh, key, high)

            botWithChips.delete(key)
        }
    }

    const part2Answer = outputs.get(0) * outputs.get(1) * outputs.get(2)
    console.log(`Part 2 answer: ${part2Answer}`)
}

part1()

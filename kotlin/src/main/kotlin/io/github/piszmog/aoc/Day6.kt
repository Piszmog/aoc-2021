package io.github.piszmog.aoc

fun main(args: Array<String>) {
}

private const val CYCLES = 9

fun getFish(input: String): Map<Int, Long> {
    val fish = mutableMapOf<Int, Long>().apply { for (i in 0 until CYCLES) this[i] = 0 }
    input.split(",").forEach { fish[it.toInt()] = fish[it.toInt()]!! + 1 }
    return fish
}

fun getFishPop(fish: Map<Int, Long>, days: Int): Long {
    val pop = fish.toMutableMap()
    for (day in 0 until days) {
        var endCycleFish = 0L
        for (cycle in 0 until CYCLES) {
            val num = pop[cycle]
            if (cycle == 0) {
                endCycleFish = num!!
            } else {
                pop[cycle - 1] = num!!
            }
        }
        pop[CYCLES - 1] = endCycleFish
        pop[CYCLES - 3] = pop[CYCLES - 3]!! + endCycleFish
    }
    return pop.values.sum()
}


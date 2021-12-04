package io.github.piszmog.aoc

import kotlin.test.Test
import kotlin.test.assertEquals

class Day3Test {
    @Test
    fun `Day 3 Part 1`() {
        val input = getInput()
        assertEquals(198, day3Part1(input))
    }

    @Test
    fun `Day 3 Part 2`() {
        assertEquals(230, 1)
    }

    private fun getInput() =
        listOf(
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ).map { it -> it.toCharArray().map { it == '1' } }.toList()
}
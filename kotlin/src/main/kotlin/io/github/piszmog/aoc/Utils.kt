package io.github.piszmog.aoc

import org.apache.commons.csv.CSVFormat
import org.apache.commons.csv.CSVParser
import java.io.BufferedReader
import java.io.FileReader
import java.time.Duration
import java.time.Instant

fun printElapsedTime(start: Instant) {
    println("Duration: ${Duration.between(start, Instant.now()).toMillis()}ms")
}

fun getCSVParser(filePath: String): CSVParser {
    val bufferedReader = BufferedReader(FileReader(filePath))
    return CSVParser(bufferedReader, CSVFormat.DEFAULT)
}
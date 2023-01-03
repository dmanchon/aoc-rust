package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strings"

	"manchon.xyz/playground/queue"
	"manchon.xyz/playground/stack"
)

//go:embed input.txt
var input string

//go:embed sample.txt
var sample string

type heightMap map[complex128]rune
type graph map[complex128][]complex128

func NewHeightMap(input string) heightMap {
	heights := make(heightMap)

	scanner := bufio.NewScanner(strings.NewReader(input))
	scanner.Split(bufio.ScanLines)

	for i := 0; scanner.Scan(); i++ {
		for j, ch := range scanner.Text() {
			heights[complex(float64(j), float64(i))] = ch
		}
	}
	return heights
}

func (m heightMap) FindExtremes() (S complex128, E complex128) {
	for k, v := range m {
		switch v {
		case 'S':
			S = k
		case 'E':
			E = k
		}
	}
	return
}

func (m heightMap) FindStarts() []complex128 {
	nodes := make([]complex128, 0)

	for k, v := range m {
		if v == 'a' || v == 'S' {
			nodes = append(nodes, k)
		}
	}
	return nodes
}

func (m heightMap) Graph() graph {
	g := make(graph)
	for k, v := range m {
		g[k] = make([]complex128, 0)
		for _, d := range [4]complex128{1 + 0i, -1 + 0i, 0 + 1i, 0 - 1i} {
			vn, exists := m[k+d]
			if !exists {
				continue
			}

			if v == 'S' && (vn-'a') <= 1 {
				g[k] = append(g[k], k+d)
			} else if vn == 'E' && ('z'-v) <= 1 {
				g[k] = append(g[k], k+d)
			} else if vn != 'E' && v != 'S' && (vn-v) <= 1 {
				g[k] = append(g[k], k+d)
			}
		}
	}
	return g
}

func contains(arr []complex128, target complex128) bool {
	for _, candidate := range arr {
		if candidate == target {
			return true
		}
	}
	return false
}

func (g graph) Dfs(start, end complex128) []complex128 {
	s := stack.NewStack[complex128]()
	visited := make([]complex128, 0)
	s.Push(start)
	for {
		node, _ := s.Peek()
		if node == end {
			return s.S
		}
		available := false
		for _, possible := range g[node] {
			if contains(visited, possible) {
				continue
			}
			s.Push(possible)
			visited = append(visited, possible)
			available = true
		}
		if !available {
			s.Pop()
		}
	}
}

func (g graph) Bfs(start, end complex128) []complex128 {
	q := queue.NewQueue[complex128]()
	parents := make(map[complex128]complex128)
	visited := make([]complex128, 0)

	q.Push(start)

	for {
		node, err := q.Pop()
		if err != nil {
			break
		}
		if node == end {
			s := stack.NewStack[complex128]()
			first, last := end, parents[end]
			for last != start {
				s.Push(first)
				first, last = last, parents[last]
			}
			s.Push(first)
			s.Push(last)

			return s.S
		}
		for _, possible := range g[node] {
			if contains(visited, possible) {
				continue
			}

			visited = append(visited, possible)
			q.Push(possible)
			parents[possible] = node
		}

	}
	return nil
}

func solve_part1(input string) int {
	heights := NewHeightMap(input)
	g := heights.Graph()
	start, end := heights.FindExtremes()
	path := g.Bfs(start, end)

	return len(path) - 1

}

func solve_part2(input string) int {
	solutions := make([]int, 0)
	heights := NewHeightMap(input)
	g := heights.Graph()
	_, end := heights.FindExtremes()
	for _, node := range heights.FindStarts() {
		path := g.Bfs(node, end)
		if len(path) > 0 {
			solutions = append(solutions, len(path)-1)
		}
	}

	solution := solutions[0]
	for i, s := range solutions {
		if s < solution {
			solution = solutions[i]
		}
	}

	return solution
}

func main() {
	fmt.Println(solve_part1(input))
	fmt.Println(solve_part2(input))
}

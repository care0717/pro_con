package main

import (
	"bufio"
	"fmt"
	"image"
	"image/color"
	"image/gif"
	"math"
	"os"
	"strconv"
	"strings"
)

type Company struct {
	X, Y, R int
}

type Rectangle struct {
	X1, Y1, X2, Y2 int
}

type Frame struct {
	Companies  []Company
	Rectangles []Rectangle
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: go run visualizer.go <input.txt> <output.gif>")
		os.Exit(1)
	}

	inputFile := os.Args[1]
	outputFile := os.Args[2]
	frames := readInput(inputFile)

	createGIF(frames, outputFile)
	fmt.Printf("GIF created: %s\n", outputFile)
}

func readInput(filename string) []Frame {
	file, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var frames []Frame

	// Read first line for n
	scanner.Scan()
	n, _ := strconv.Atoi(strings.TrimSpace(scanner.Text()))

	var companies []Company

	// Read companies (input)
	for i := 0; i < n; i++ {
		scanner.Scan()
		parts := strings.Fields(scanner.Text())
		x, _ := strconv.Atoi(parts[0])
		y, _ := strconv.Atoi(parts[1])
		r, _ := strconv.Atoi(parts[2])
		companies = append(companies, Company{X: x, Y: y, R: r})
	}

	// Read K outputs
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}

		var rectangles []Rectangle

		// Read rectangles for this output
		for i := 0; i < n; i++ {
			if i == 0 {
				// Use the current line for first rectangle
				parts := strings.Fields(line)
				x1, _ := strconv.Atoi(parts[0])
				y1, _ := strconv.Atoi(parts[1])
				x2, _ := strconv.Atoi(parts[2])
				y2, _ := strconv.Atoi(parts[3])
				rectangles = append(rectangles, Rectangle{X1: x1, Y1: y1, X2: x2, Y2: y2})
			} else {
				scanner.Scan()
				parts := strings.Fields(scanner.Text())
				x1, _ := strconv.Atoi(parts[0])
				y1, _ := strconv.Atoi(parts[1])
				x2, _ := strconv.Atoi(parts[2])
				y2, _ := strconv.Atoi(parts[3])
				rectangles = append(rectangles, Rectangle{X1: x1, Y1: y1, X2: x2, Y2: y2})
			}
		}

		frames = append(frames, Frame{Companies: companies, Rectangles: rectangles})
	}

	return frames
}

func createGIF(frames []Frame, filename string) {
	const (
		width      = 800
		height     = 850  // Increased height for header
		headerSize = 50   // Header area for score
		gameHeight = 800  // Game area height
		scale      = 800.0 / 10000.0
	)

	var images []*image.Paletted
	var delays []int

	// Create gradient palette with more colors for smoother transitions
	palette := make(color.Palette, 256)
	palette[0] = color.RGBA{255, 255, 255, 255} // white (background)
	palette[1] = color.RGBA{0, 0, 0, 255}       // black (border/text)
	palette[2] = color.RGBA{0, 255, 0, 255}     // green (input points)

	// Generate gradient from blue (score=0) to pink (score=1) to red (score>1)
	for i := 3; i < 256; i++ {
		// Map index to score (0.0 to 2.0)
		score := float64(i-3) / 252.0 * 2.0
		if score <= 1.0 {
			// Blue to Pink gradient
			t := score             // 0.0 to 1.0
			r := uint8(t * 255)    // 0 to 255
			g := uint8(t * 192)    // 0 to 192
			b := uint8(255 - t*52) // 255 to 203
			palette[i] = color.RGBA{r, g, b, 255}
		} else {
			// Pink to Red gradient
			t := (score - 1.0) // 0.0 to 1.0 for scores > 1
			if t > 1.0 {
				t = 1.0
			}
			r := uint8(255)
			g := uint8(192 - t*192) // 192 to 0
			b := uint8(203 - t*203) // 203 to 0
			palette[i] = color.RGBA{r, g, b, 255}
		}
	}

	for _, frame := range frames {
		img := image.NewPaletted(image.Rect(0, 0, width, height), palette)

		// Fill background
		for y := 0; y < height; y++ {
			for x := 0; x < width; x++ {
				img.Set(x, y, palette[0]) // white
			}
		}
		
		// Draw header background (light gray)
		headerColor := color.RGBA{240, 240, 240, 255}
		for y := 0; y < headerSize; y++ {
			for x := 0; x < width; x++ {
				img.Set(x, y, headerColor)
			}
		}
		
		// Draw header separator line
		for x := 0; x < width; x++ {
			img.Set(x, headerSize-1, palette[1]) // black line
		}

		// Draw rectangles
		for i, rect := range frame.Rectangles {
			company := frame.Companies[i]
			c := getColor(company, rect, palette)

			x1 := int(float64(rect.X1) * scale)
			y1 := int(float64(rect.Y1) * scale) + headerSize  // Offset by header
			x2 := int(float64(rect.X2) * scale)
			y2 := int(float64(rect.Y2) * scale) + headerSize  // Offset by header

			// Fill rectangle
			for y := y1; y < y2 && y < height; y++ {
				for x := x1; x < x2 && x < width; x++ {
					if x >= 0 && y >= headerSize {  // Don't draw in header area
						img.Set(x, y, c)
					}
				}
			}

			// Draw border
			drawBorder(img, x1, y1, x2, y2, palette[1], width, height, headerSize)

			// Draw index number and score in the center of rectangle
			centerX := (x1 + x2) / 2
			centerY := (y1 + y2) / 2
			
			// Draw index number above center
			drawNumber(img, centerX, centerY-15, i, palette[1], width, height)
			
			// Calculate and draw individual score below center
			individualScore := calculateIndividualScore(company, rect)
			drawScore4Digits(img, centerX, centerY+15, individualScore, palette[1], width, height)
		}

		// Draw input points (green) and connection lines
		for i, company := range frame.Companies {
			pointX := int(float64(company.X) * scale)
			pointY := int(float64(company.Y) * scale) + headerSize  // Offset by header
			drawPoint(img, pointX, pointY, palette[2], width, height) // green
			
			// Draw connection line to rectangle center
			if i < len(frame.Rectangles) {
				rect := frame.Rectangles[i]
				rectCenterX := int(float64(rect.X1+rect.X2) * scale / 2)
				rectCenterY := int(float64(rect.Y1+rect.Y2) * scale / 2) + headerSize
				drawLine(img, pointX, pointY, rectCenterX, rectCenterY, palette[1], width, height)
			}
		}

		// Validate rectangles and draw total score in header
		if valid, errorMsg := validateRectangles(frame.Companies, frame.Rectangles); !valid {
			fmt.Printf("Warning: %s\n", errorMsg)
		}
		totalScore := calculateTotalScore(frame)
		drawScore(img, totalScore, palette[1], width, height)

		images = append(images, img)
		delays = append(delays, 50) // 0.5 second delay
	}

	// Save GIF
	f, err := os.Create(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	// Create GIF with proper settings
	anim := &gif.GIF{
		Image:     images,
		Delay:     delays,
		LoopCount: 0, // Infinite loop
	}
	
	err = gif.EncodeAll(f, anim)
	if err != nil {
		panic(err)
	}
}

func getColor(company Company, rect Rectangle, palette color.Palette) color.Color {
	// Check if rectangle contains target point (x+0.5, y+0.5)
	targetX := float64(company.X) + 0.5
	targetY := float64(company.Y) + 0.5

	if float64(rect.X1) <= targetX && targetX < float64(rect.X2) &&
		float64(rect.Y1) <= targetY && targetY < float64(rect.Y2) {

		// Rectangle contains target point, calculate score
		area := (rect.X2 - rect.X1) * (rect.Y2 - rect.Y1)
		minVal := float64(minInt(area, company.R))
		maxVal := float64(maxInt(area, company.R))

		score := 1.0 - math.Pow(1.0-minVal/maxVal, 2.0)

		// Map score to palette index (3-255)
		colorIndex := int(score/2.0*252.0) + 3
		if colorIndex > 255 {
			colorIndex = 255
		}
		if colorIndex < 3 {
			colorIndex = 3
		}
		return palette[colorIndex]
	} else {
		// Rectangle doesn't contain target point, use blue (score=0)
		return palette[3] // darkest blue
	}
}

func drawBorder(img *image.Paletted, x1, y1, x2, y2 int, borderColor color.Color, width, height, headerSize int) {
	// Top and bottom borders
	for x := maxInt(0, x1); x < minInt(width, x2); x++ {
		if y1 >= headerSize && y1 < height {
			img.Set(x, y1, borderColor)
		}
		if y2-1 >= headerSize && y2-1 < height {
			img.Set(x, y2-1, borderColor)
		}
	}

	// Left and right borders
	for y := maxInt(headerSize, y1); y < minInt(height, y2); y++ {
		if x1 >= 0 && x1 < width {
			img.Set(x1, y, borderColor)
		}
		if x2-1 >= 0 && x2-1 < width {
			img.Set(x2-1, y, borderColor)
		}
	}
}

func drawPoint(img *image.Paletted, x, y int, pointColor color.Color, width, height int) {
	// Draw a smaller point (3x3)
	radius := 2
	for dy := -radius; dy <= radius; dy++ {
		for dx := -radius; dx <= radius; dx++ {
			if dx*dx + dy*dy <= radius*radius {
				px, py := x+dx, y+dy
				if px >= 0 && px < width && py >= 0 && py < height {
					img.Set(px, py, pointColor)
				}
			}
		}
	}
}

func drawNumber(img *image.Paletted, x, y, number int, textColor color.Color, width, height int) {
	// Draw number using simple digit patterns (2x size)
	digits := getDigits(number)
	startX := x - (len(digits)*7)/2 // Center the number (2x size: 6 width + 1 spacing)

	for i, digit := range digits {
		digitX := startX + i*7 // 7 pixels per digit (6 width + 1 spacing)
		drawDigit(img, digitX, y, digit, textColor, width, height)
	}
}

func getDigits(n int) []int {
	if n == 0 {
		return []int{0}
	}
	var digits []int
	for n > 0 {
		digits = append([]int{n % 10}, digits...)
		n /= 10
	}
	return digits
}

func drawDigit(img *image.Paletted, x, y, digit int, textColor color.Color, width, height int) {
	// Simple 3x5 digit patterns
	patterns := map[int][]string{
		0: {"###", "# #", "# #", "# #", "###"},
		1: {" # ", "## ", " # ", " # ", "###"},
		2: {"###", "  #", "###", "#  ", "###"},
		3: {"###", "  #", "###", "  #", "###"},
		4: {"# #", "# #", "###", "  #", "  #"},
		5: {"###", "#  ", "###", "  #", "###"},
		6: {"###", "#  ", "###", "# #", "###"},
		7: {"###", "  #", "  #", "  #", "  #"},
		8: {"###", "# #", "###", "# #", "###"},
		9: {"###", "# #", "###", "  #", "###"},
	}

	pattern := patterns[digit]
	for row, line := range pattern {
		for col, char := range line {
			if char == '#' {
				// Draw 2x2 pixels for each digit pixel (2x size)
				for dy := 0; dy < 2; dy++ {
					for dx := 0; dx < 2; dx++ {
						px, py := x+col*2+dx-3, y+row*2-5+dy // Center around (x,y)
						if px >= 0 && px < width && py >= 0 && py < height {
							img.Set(px, py, textColor)
						}
					}
				}
			}
		}
	}
}

func minInt(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func calculateIndividualScore(company Company, rect Rectangle) float64 {
	// Check if rectangle contains target point (x+0.5, y+0.5)
	targetX := float64(company.X) + 0.5
	targetY := float64(company.Y) + 0.5
	
	if float64(rect.X1) <= targetX && targetX < float64(rect.X2) &&
		float64(rect.Y1) <= targetY && targetY < float64(rect.Y2) {
		
		// Rectangle contains target point, calculate score
		area := (rect.X2 - rect.X1) * (rect.Y2 - rect.Y1)
		minVal := float64(minInt(area, company.R))
		maxVal := float64(maxInt(area, company.R))
		
		return 1.0 - math.Pow(1.0-minVal/maxVal, 2.0)
	}
	// If rectangle doesn't contain target point, score is 0
	return 0.0
}

func drawScore4Digits(img *image.Paletted, x, y int, score float64, textColor color.Color, width, height int) {
	// Format score to 4 significant digits
	var scoreText string
	if score == 0.0 {
		scoreText = "0.000"
	} else if score >= 1.0 {
		scoreText = fmt.Sprintf("%.3f", score)
	} else if score >= 0.1 {
		scoreText = fmt.Sprintf("%.3f", score)
	} else if score >= 0.01 {
		scoreText = fmt.Sprintf("%.4f", score)[0:6] // 0.xxxx
	} else {
		scoreText = fmt.Sprintf("%.4g", score)
	}
	
	// Draw text centered with smaller font
	textWidth := len(scoreText) * 7 // 7 pixels per character (2x size: 6 width + 1 spacing)
	startX := x - textWidth/2
	drawTextSmall(img, startX, y, scoreText, textColor, width, height)
}

func calculateTotalScore(frame Frame) int64 {
	var totalScore float64
	n := len(frame.Companies)

	for i, rect := range frame.Rectangles {
		company := frame.Companies[i]

		// Check if rectangle contains target point (x+0.5, y+0.5)
		targetX := float64(company.X) + 0.5
		targetY := float64(company.Y) + 0.5

		if float64(rect.X1) <= targetX && targetX < float64(rect.X2) &&
			float64(rect.Y1) <= targetY && targetY < float64(rect.Y2) {

			// Rectangle contains target point, calculate score
			area := (rect.X2 - rect.X1) * (rect.Y2 - rect.Y1)
			minVal := float64(minInt(area, company.R))
			maxVal := float64(maxInt(area, company.R))

			score := 1.0 - math.Pow(1.0-minVal/maxVal, 2.0)
			totalScore += score
		}
		// If rectangle doesn't contain target point, score is 0
	}

	// Calculate final score: (totalScore / n) * 10^9
	finalScore := (totalScore / float64(n)) * 1000000000
	return int64(finalScore)
}

func drawScore(img *image.Paletted, score int64, textColor color.Color, width, height int) {
	// Draw score in header area
	scoreText := fmt.Sprintf("Score: %d", score)
	drawText(img, 10, 25, scoreText, textColor, width, height)
}

func drawText(img *image.Paletted, x, y int, text string, textColor color.Color, width, height int) {
	// Simple text drawing - each character is 18 pixels wide (tripled)
	for i, char := range text {
		charX := x + i*21 // 18 pixels + 3 spacing
		drawChar(img, charX, y, char, textColor, width, height)
	}
}

func drawTextSmall(img *image.Paletted, x, y int, text string, textColor color.Color, width, height int) {
	// Simple text drawing - each character is 6 pixels wide (2x)
	for i, char := range text {
		charX := x + i*7 // 6 pixels + 1 spacing
		drawCharSmall(img, charX, y, char, textColor, width, height)
	}
}

func drawChar(img *image.Paletted, x, y int, char rune, textColor color.Color, width, height int) {
	// Simple character patterns (5x7)
	patterns := map[rune][]string{
		'S': {"###", "#  ", "###", "  #", "###"},
		'c': {"###", "#  ", "#  ", "#  ", "###"},
		'o': {"###", "# #", "# #", "# #", "###"},
		'r': {"###", "# #", "###", "# #", "# #"},
		'e': {"###", "#  ", "###", "#  ", "###"},
		':': {"   ", " # ", "   ", " # ", "   "},
		' ': {"   ", "   ", "   ", "   ", "   "},
		'.': {"   ", "   ", "   ", "   ", " # "},
		'0': {"###", "# #", "# #", "# #", "###"},
		'1': {" # ", "## ", " # ", " # ", "###"},
		'2': {"###", "  #", "###", "#  ", "###"},
		'3': {"###", "  #", "###", "  #", "###"},
		'4': {"# #", "# #", "###", "  #", "  #"},
		'5': {"###", "#  ", "###", "  #", "###"},
		'6': {"###", "#  ", "###", "# #", "###"},
		'7': {"###", "  #", "  #", "  #", "  #"},
		'8': {"###", "# #", "###", "# #", "###"},
		'9': {"###", "# #", "###", "  #", "###"},
	}

	pattern, exists := patterns[char]
	if !exists {
		return
	}

	for row, line := range pattern {
		for col, pixel := range line {
			if pixel == '#' {
				// Draw 3x3 pixels for each character pixel to triple the size
				for dy := 0; dy < 3; dy++ {
					for dx := 0; dx < 3; dx++ {
						px, py := x+col*3+dx, y+row*3-6+dy
						if px >= 0 && px < width && py >= 0 && py < height {
							img.Set(px, py, textColor)
						}
					}
				}
			}
		}
	}
}

func drawCharSmall(img *image.Paletted, x, y int, char rune, textColor color.Color, width, height int) {
	// Simple character patterns (5x7) - 2x size
	patterns := map[rune][]string{
		'S': {"###", "#  ", "###", "  #", "###"},
		'c': {"###", "#  ", "#  ", "#  ", "###"},
		'o': {"###", "# #", "# #", "# #", "###"},
		'r': {"###", "# #", "###", "# #", "# #"},
		'e': {"###", "#  ", "###", "#  ", "###"},
		':': {"   ", " # ", "   ", " # ", "   "},
		' ': {"   ", "   ", "   ", "   ", "   "},
		'.': {"   ", "   ", "   ", "   ", " # "},
		'0': {"###", "# #", "# #", "# #", "###"},
		'1': {" # ", "## ", " # ", " # ", "###"},
		'2': {"###", "  #", "###", "#  ", "###"},
		'3': {"###", "  #", "###", "  #", "###"},
		'4': {"# #", "# #", "###", "  #", "  #"},
		'5': {"###", "#  ", "###", "  #", "###"},
		'6': {"###", "#  ", "###", "# #", "###"},
		'7': {"###", "  #", "  #", "  #", "  #"},
		'8': {"###", "# #", "###", "# #", "###"},
		'9': {"###", "# #", "###", "  #", "###"},
	}
	
	pattern, exists := patterns[char]
	if !exists {
		return
	}
	
	for row, line := range pattern {
		for col, pixel := range line {
			if pixel == '#' {
				// Draw 2x2 pixels for each character pixel (2x size)
				for dy := 0; dy < 2; dy++ {
					for dx := 0; dx < 2; dx++ {
						px, py := x+col*2+dx, y+row*2-5+dy
						if px >= 0 && px < width && py >= 0 && py < height {
							img.Set(px, py, textColor)
						}
					}
				}
			}
		}
	}
}

func drawLine(img *image.Paletted, x1, y1, x2, y2 int, lineColor color.Color, width, height int) {
	// Simple line drawing using Bresenham's algorithm
	dx := abs(x2 - x1)
	dy := abs(y2 - y1)
	sx := 1
	if x1 > x2 {
		sx = -1
	}
	sy := 1
	if y1 > y2 {
		sy = -1
	}
	
	err := dx - dy
	x, y := x1, y1
	
	for {
		if x >= 0 && x < width && y >= 0 && y < height {
			img.Set(x, y, lineColor)
		}
		
		if x == x2 && y == y2 {
			break
		}
		
		e2 := 2 * err
		if e2 > -dy {
			err -= dy
			x += sx
		}
		if e2 < dx {
			err += dx
			y += sy
		}
	}
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func validateRectangles(companies []Company, rectangles []Rectangle) (bool, string) {
	// Check if rectangles are valid according to the problem constraints
	for i, rect := range rectangles {
		// Check bounds
		if !checkBounds(rect) {
			return false, fmt.Sprintf("Rectangle %d is out of bounds", i)
		}
		
		// Check positive area
		if rect.X1 >= rect.X2 || rect.Y1 >= rect.Y2 {
			return false, fmt.Sprintf("Rectangle %d has non-positive area", i)
		}
		
		// Check if rectangle contains target point
		company := companies[i]
		if !rect.includesPoint(company.X, company.Y) {
			return false, fmt.Sprintf("Rectangle %d does not contain target point (%d,%d)", i, company.X, company.Y)
		}
		
		// Check overlap with other rectangles
		for j := i + 1; j < len(rectangles); j++ {
			if checkOverlap(rect, rectangles[j]) {
				return false, fmt.Sprintf("Rectangle %d overlaps with rectangle %d", i, j)
			}
		}
	}
	return true, ""
}

func checkBounds(rect Rectangle) bool {
	return rect.X1 >= 0 && rect.Y1 >= 0 && rect.X2 <= 10000 && rect.Y2 <= 10000
}

func checkOverlap(r1, r2 Rectangle) bool {
	return maxInt(r1.X1, r2.X1) < minInt(r1.X2, r2.X2) && maxInt(r1.Y1, r2.Y1) < minInt(r1.Y2, r2.Y2)
}

func (r Rectangle) includesPoint(x, y int) bool {
	return r.X1 <= x && x < r.X2 && r.Y1 <= y && y < r.Y2
}

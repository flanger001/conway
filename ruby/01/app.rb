class Cell
  attr_accessor :neighbors_count

  def initialize(status = :dead)
    @status = status
    @neighbors_count = 0
  end

  def kill = @status = :dead
  def resurrect = @status = :alive

  def alive? = @status == :alive
  def dead? = @status == :dead

  def to_s = alive? ? "O" : " "
end

class Board
  LIVE_NEIGHBORS_RANGE = (2..3).freeze

  def initialize(length: 10, width: 10)
    @length, @width = length, width
    @grid = Array.new(length) do
      Array.new(width) do
        Cell.new(rand < 0.3 ? :alive : :dead)
      end
    end
  end

  def mark
    @grid.each_with_index do |line, y|
      line.each_with_index do |cell, x|
        mark_neighbors(cell, x, y)
      end
    end
  end

  def sweep
    @grid.each_entry do |line|
      line.each_entry do |cell|
        if cell.alive? && LIVE_NEIGHBORS_RANGE.cover?(cell.neighbors_count)
          nil
        elsif cell.dead? && cell.neighbors_count == 3
          cell.resurrect
        else
          cell.kill
        end
        cell.neighbors_count = 0
      end
    end
  end

  def print
    @grid.each do |line|
      Kernel.print(line.reduce("") { |str, l| str << "#{l}" << "" })
      puts
    end
  end

  private

  def mark_neighbors(cell, x, y)
    neighbors = []

    neighbors.concat(neighbors_adjacent(x, y))

    if y == 0 # top edge
      neighbors.concat(neighbors_below(x, y))
    elsif y < @length - 1 # not at bottom edge
      neighbors.concat(neighbors_above(x, y))
      neighbors.concat(neighbors_below(x, y))
    else
      neighbors.concat(neighbors_above(x, y))
    end

    cell.neighbors_count = neighbors.compact.reduce(0) { |sum, c| sum += (c.alive? ? 1 : 0) }
  end

  def neighbors_above(x, y)
    [(@grid[y - 1][x - 1] if x > 0), @grid[y - 1][x], @grid[y - 1][x + 1]]
  end

  def neighbors_below(x, y)
    [(@grid[y + 1][x - 1] if x > 0), @grid[y + 1][x], @grid[y + 1][x + 1]]
  end

  def neighbors_adjacent(x, y)
    [(@grid[y][x - 1] if x > 0), @grid[y][x + 1]]
  end
end

class Game
  def initialize(board, frame_rate = 0.5)
    @board, @frame_rate = board, frame_rate
  end

  def play
    loop do
      print("\033c")
      @board.print
      @board.mark
      @board.sweep
      sleep(@frame_rate)
    end
  end
end

board = Board.new(length: 50, width: 150)
game = Game.new(board)
game.play

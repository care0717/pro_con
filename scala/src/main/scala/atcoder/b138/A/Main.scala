package atcoder.b138.A

object Main {
  def main(args: Array[String]) {
    val input = io.Source.stdin.getLines().mkString("\n");
    println(solve(input))

    def solve(input: String): String = {
      val a :: s :: _ = input.split("\n").toList
      if (a.toInt >= 3200) {
        s
      } else {
        "red"
      }
    }
  }
}

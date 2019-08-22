package atcoder

object Template {
  def main(args: Array[String]) {
    if (sys.env.getOrElse("TEST", "") == "1") {
      println(test())
    } else {
      val input = io.Source.stdin.getLines().mkString("\n")
      println(solve(input).trim())
    }
  }

  def solve(input: String): String = {
    input
  }

  val tests = Seq(
    """10 1""" -> """11"""
  )

  def test(): String = {
    tests
      .map { case (i, o) => (i.trim(), o.trim()) }
      .zipWithIndex
      .map {
        case ((input, outputExpect), i) => {
          val output = solve(input).trim();
          s"test${i + 1}:" + (if (output == outputExpect) {
                                "Passed"
                              } else {
                                s"Failed\nexpect:\n$outputExpect\noutput:\n$output"
                              })
        }
      }
      .mkString("\n")
  }
}

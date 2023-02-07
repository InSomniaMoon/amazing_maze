package enums

import enums.Exploration.*

import scala.::
import scala.collection.mutable.ListBuffer

enum Maze:
  case Branch(label: String, left: Maze, right: Maze, var status: Exploration = UnExplored)
  case Leaf(label: String)

  def explore_old: List[String] = this match
    case branch@Branch(label, left, right, status) =>
      status match
        case UnExplored => branch.status = Explored; label :: left.explore_old ::: right.explore_old
        case Explored => List(label)
    case Leaf(label) => List(label)

  def explore(trace: ListBuffer[String]): Unit =
    this match
      case branch@Branch(label, left, right, status) =>
        status match
          case UnExplored => branch.status = Explored
            trace += label
            left.explore(trace)
            right.explore(trace)
          case Explored => trace += label
      case Leaf(label) => trace += label

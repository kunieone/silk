// ignore_for_file: public_member_api_docs, sort_constructors_first
/*游戏运行，自己有三种动作 聚(a) 挡(q,w,e,r,t(分别对应 1，2,3,4,5 级的挡） 波（1,2,3,4，5这几个数字键，分别对应 1,2,3,4，5 的级别的波）
每次按下技能，然后机器也会出招。机器的出招是随机的，但是必须按规则（聚气不够不能出更高级别的波）
按下之后 双方人机的出招都会被输出，然后 judger 播报双方的战况。接下来下一回合...
这样持续。 */

import 'dart:collection';
import 'dart:io';
import 'dart:math';

class RoundInfoLog {
  DateTime time;
  Player gamer1;
  Player gamer2;
  String info;
  RoundInfoLog(
      {required this.time,
      required this.gamer1,
      required this.gamer2,
      required this.info});
}

enum SkilType {
  ju("聚气", ["聚"]),
  dang("格挡", ["挡", "中挡", "大挡", "超挡", "极挡"]),
  bo("发波", ["波", "中波", "大波", "超波", "极波"]);

  const SkilType(this.label, this.extraNames);
  final List<String> extraNames;
  final String label;
}

class Action {
  final SkilType skilType;
  final int level;
  const Action(this.skilType, this.level);

  @override
  String toString() {
    return skilType.extraNames[level - 1];
  }
}

class Player {
  int ju = 0;
  int? defense;

  Queue<Action> actions = Queue();

  /// 聚气
  juAction(int level) {
    defense = null;
    ju++;
    actions.add(Action(SkilType.ju, level));
  }

  /// 格挡
  dangAction(int level) {
    var consume = level - 1;
    // 消耗聚的个数
    if (ju < consume) {
      throw Error();
    } else {
      ju -= consume;
    }

    defense = level;

    actions.add(Action(SkilType.dang, level));
  }

  /// 发波,交给评审处理，因为可能先后顺序问题
  boAction(int level) {
    defense = null;
    // 消耗聚的个数
    if (ju < level) {
      throw Error();
    } else {
      ju -= level;
    }
    actions.add(Action(SkilType.bo, level));
  }
}

class Game {
  final Player human = Player();
  final Player machine = Player();
  final Judger judger = Judger();
  final logs = <RoundInfoLog>[];
  bool isOver = false;

  void start() {
    while (!isOver) {
      // 枚举动作与对应的方法
      Map<String, void Function()> actionMap = {
        'a': () => human.juAction(1),
        's': () => human.dangAction(1),
        'd': () => human.dangAction(2),
        'z': () => human.dangAction(3),
        'x': () => human.dangAction(4),
        'c': () => human.dangAction(5),
        '1': () => human.boAction(1),
        '2': () => human.boAction(2),
        '3': () => human.boAction(3),
        '4': () => human.boAction(4),
        '5': () => human.boAction(5),
      };

      // 获取人类玩家的输入，并调用相应的方法
      String input = String.fromCharCode(stdin.readByteSync()).toString();
      if (input == "q") {
        exit(1);
      }
      if (actionMap.keys.contains(input) == false) {
        continue;
      }
      if (!allowUse(human, input)) {
        print("您的聚气值{${human.ju}}不够!");
        continue;
      }
      actionMap[input]?.call();

      // 随机生成机器玩家的出招，并根据出招调用相应的方法
      Random random = Random();
      int action = random.nextInt(3); // 0 表示聚，1 表示挡，2 表示波

      if (machine.ju == 0) {
        if (random.nextInt(2) == 1) {
          machine.juAction(1);
        } else {
          machine.dangAction(1);
        }
      } else {
        var limit = max(machine.ju, 1);
        int level = random.nextInt(limit) + 1; // 1 到 5 表示不同的等级
        switch (action) {
          case 0:
            machine.juAction(1);
            break;
          case 1:
            machine.dangAction(level);
            break;
          case 2:
            machine.boAction(level);
            break;
        }
      }
      var info = RoundInfoLog(
          time: DateTime.now(), gamer1: human, gamer2: machine, info: "");
      stdout.write(DateTime.now().toLocal());
      stdout.write("  \n");
      // 打印双方的出招
      info.info += '你 ${human.actions} ->>     <<- ${machine.actions} 机器';

      // 调用 Judger 的方法来判断游戏的结果，并打印相应的信息
      int result = judger.judge(human, machine);

      switch (result) {
        case 0:
          info.info += ("\n本回合平局。你:聚气点数(${human.ju}),机器人:聚气点数(${machine.ju})");
          break;
        case 3:
          info.info += ('\n恭喜你，你赢得了游戏！');
          isOver = true;
          break;
        case 4:
          info.info += ('\n很遗憾，你输掉了游戏。');
          isOver = true;
          break;
      }
      info.info += ('\n');
      print(info.info);
      logs.add(info);
    }
  }

  bool allowUse(Player human, String input) {
    var consume = {
      "a": 0,
      "s": 1,
      "d": 2,
      "z": 3,
      "x": 4,
      "c": 5,
      "1": 1,
      "2": 2,
      "3": 3,
      "4": 4,
      "5": 5,
    };
    if (consume[input] == null) return false;
    return consume[input]! <= human.ju;
  }
}

// Judger 类用来判断游戏的结果
class Judger {
  Judger();
  int? receivedJuLevel;
  // 定义一个 judge 方法，接受两个 Player 对象作为参数，返回一个 int 值作为结果
  // 返回值的含义如下：
  // 0 表示本回合平局
  // 3 表示人类玩家赢得了游戏
  // 4 表示机器玩家赢得了游戏
  int judge(Player human, Player machine) {
    Action actionH = human.actions.removeFirst();
    Action actionM = machine.actions.removeFirst();
    if (actionH.skilType == SkilType.dang &&
            actionM.skilType == SkilType.dang ||
        actionM.skilType == SkilType.ju && actionH.skilType == SkilType.ju) {
      return 0;
    } else if (actionH.skilType == SkilType.bo &&
        actionM.skilType == SkilType.ju) {
      return 3;
    } else if (actionM.skilType == SkilType.bo &&
        actionH.skilType == SkilType.ju) {
      return 4;
    } else if (actionH.skilType == SkilType.bo &&
        actionM.skilType == SkilType.bo) {
      return actionH.level == actionM.level
          ? 0
          : actionH.level > actionM.level
              ? 3
              : 4;
    } else if (actionH.skilType == SkilType.bo &&
        actionM.skilType == SkilType.dang) {
      return actionH.level == actionM.level
          ? 0
          : actionH.level > actionM.level
              ? 3
              : 0;
    } else if (actionH.skilType == SkilType.dang &&
        actionM.skilType == SkilType.bo) {
      return actionH.level == actionM.level
          ? 0
          : actionH.level > actionM.level
              ? 0
              : 4;
    }
    return 0;
  }
}

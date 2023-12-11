/* 聚挡波是一个风靡千禧年的全国小孩子见的游戏。游戏每回合双方同时出招，聚挡波，聚可以积攒气，挡可以格挡，波是攻击。

攒气的数量可以发招更厉害的格挡和攻击。
挡的一级二级三级分别消耗(0,1,2,3,4)
波的一级二级三级..分别消耗(1,2,3,4,5）
以下情况会输掉：
自己没有出挡，对方波了
自己出了挡，但是没有挡住（1挡可以挡住同级别的波）
下面是 dart 代码的实现。
enum SkilType {
  ju,
  dang,
  bo;
}

enum BO {
  normal(1),
  big(2),
  huge(3),
  supreme(4),
  extreme(5);

  final int level;
  const BO(int this.level);
}

class Player {
  int health = 1;
  int ju = 0;
  int? dang;
  SkilType? skilThisTurn;

  /// 聚气
  juAction(int level) {
    skilThisTurn = SkilType.ju;
    dang = null;
    ju++;
  }

  /// 格挡
  dangAction(int level) {
    skilThisTurn = SkilType.dang;
    dang = level;
  }

  /// 发波
  boAction(Player target, int level) {
    skilThisTurn = SkilType.bo;
    dang = null;
    if (target.dang != null) {
      // 是否格挡成功
      var val = target.dang! - level;
      if (val == 0) {
        print("刚好格挡住");
      } else if (val > 0) {
        print("完美格挡");
      } else {
        print("绝杀");
      }
    }
    target.health - (level - (target.dang ?? 0));
  }
}

class Game {}

class Judger {}
请你补充 Game 和 Judger。
由于他并不是顺序的回合制（双方是同时出招的，毕竟如果先后出招，那所有的波都可以挡了），我想做成 cli游戏，那就引入了一个 judger，用来处理双方出招的顺序问题和判别失败与否。
请你完成这个命令行游戏。
我需要 游戏运行，自己有三种动作 聚(a) 挡(q,w,e,r,t(分别对应 1，2,3,4,5 级的挡） 波（1,2,3,4，5这几个数字键，分别对应 1,2,3,4，5 的级别的波）
每次按下技能，然后机器也会出招。机器的出招是随机的，但是必须按规则（聚气不够不能出更高级别的波）
按下之后 双方人机的出招都会被输出，然后 judger 播报双方的战况。接下来下一回合...
这样持续。 */
import 'dart:io';
import 'dart:math';

enum SkilType {
  ju,
  dang,
  bo;
}

enum BO {
  normal(1),
  big(2),
  huge(3),
  supreme(4),
  extreme(5);

  final int level;
  const BO(int this.level);
}

class Player {
  int health = 1;
  int ju = 0;
  int? dang;
  SkilType? skilThisTurn;

  /// 聚气
  juAction(int level) {
    skilThisTurn = SkilType.ju;
    dang = null;
    ju++;
  }

  /// 格挡
  dangAction(int level) {
    skilThisTurn = SkilType.dang;

    var consume = level - 1;
    // 消耗聚的个数
    if (ju < consume) {
      throw Error();
    } else {
      ju -= consume;
    }

    dang = level;
  }

  /// 发波
  boAction(Player target, int level) {
    skilThisTurn = SkilType.bo;
    dang = null;
    // 消耗聚的个数
    if (ju < level) {
      throw Error();
    } else {
      ju -= level;
    }
    if (target.dang != null) {
      // 是否格挡成功
      var val = target.dang! - level;
      if (val == 0) {
        print("刚好格挡住");
      } else if (val > 0) {
        print("完美格挡");
      } else {
        print("绝杀");
      }
    }
    target.health - (level - (target.dang ?? 0));
  }
}

// Game 类用来管理游戏的逻辑和状态
class Game {
  // 定义两个玩家，一个是人类玩家，一个是机器玩家
  final Player human = Player();
  final Player machine = Player();

  // 定义一个 Judger 对象，用来判断游戏的结果
  final Judger judger = Judger();

  // 定义一个 bool 变量，用来表示游戏是否结束
  bool isOver = false;

  // 定义一个构造函数，初始化游戏的状态

  // 定义一个 start 方法，用来开始游戏
  void start() {
    // 打印欢迎信息和游戏说明
    print('欢迎来到聚挡波游戏！你将和机器玩家对战，看谁能先打败对方。');
    print('游戏规则如下：');
    print('每回合双方同时出招，聚挡波，聚可以积攒气，挡可以格挡，波是攻击。');
    print('攒气的数量可以发招更厉害的格挡和攻击。');
    print('挡的一级二级三级分别消耗(0,1,2,3,4)');
    print('波的一级二级三级..分别消耗(1,2,3,4,5）');
    print('以下情况会输掉：');
    print('自己没有出挡，对方波了');
    print('自己出了挡，但是没有挡住（1挡可以挡住同级别的波）');
    print('你有三种动作：聚(a) 挡(q,w,e,r,t) 波(1,2,3,4,5)');
    print('请按下相应的键来出招，然后按回车键确认。');

    // 进入游戏循环，直到游戏结束
    while (!isOver) {
      // 获取人类玩家的输入，并根据输入调用相应的方法
      String input = stdin.readLineSync()!;
      switch (input) {
        case 'a':
          human.juAction(1);
          break;
        case 'q':
          human.dangAction(1);
          break;
        case 'w':
          human.dangAction(2);
          break;
        case 'e':
          human.dangAction(3);
          break;
        case 'r':
          human.dangAction(4);
          break;
        case 't':
          human.dangAction(5);
          break;
        case '1':
          human.boAction(machine, 1);
          break;
        case '2':
          human.boAction(machine, 2);
          break;
        case '3':
          human.boAction(machine, 3);
          break;
        case '4':
          human.boAction(machine, 4);
          break;
        case '5':
          human.boAction(machine, 5);
          break;
        default:
          print('无效的输入，请重新输入。');
          continue;
      }

      // 随机生成机器玩家的出招，并根据出招调用相应的方法
      Random random = Random();
      int action = random.nextInt(3); // 0 表示聚，1 表示挡，2 表示波
      int level = random.nextInt(5) + 1; // 1 到 5 表示不同的等级
      switch (action) {
        case 0:
          machine.juAction(level);
          break;
        case 1:
          machine.dangAction(level);
          break;
        case 2:
          machine.boAction(human, level);
          break;
      }

      // 打印双方的出招
      print('你的出招是：${human.skilThisTurn}');
      print('机器的出招是：${machine.skilThisTurn}');

      // 调用 Judger 的方法来判断游戏的结果，并打印相应的信息
      int result = judger.judge(human, machine);
      switch (result) {
        case 0:
          print('本回合平局。你:聚气点数(${human.ju}),机器人:聚气点数(${machine.ju})');
          break;
        case 3:
          print('恭喜你，你赢得了游戏！');
          isOver = true;
          break;
        case 4:
          print('很遗憾，你输掉了游戏。');
          isOver = true;
          break;
      }
    }
  }
}

// Judger 类用来判断游戏的结果
class Judger {
  // 定义一个构造函数，无需初始化任何内容
  Judger();

  // 定义一个 judge 方法，接受两个 Player 对象作为参数，返回一个 int 值作为结果
  // 返回值的含义如下：
  // 0 表示本回合平局
  // 3 表示人类玩家赢得了游戏
  // 4 表示机器玩家赢得了游戏
  int judge(Player human, Player machine) {
    if (human.health < 0) {
      return 3;
    } else if (machine.health < 0) {
      return 4;
    } else {
      return 0;
    }
  }
}

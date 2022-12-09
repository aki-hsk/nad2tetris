@8182
D=A
@max_num_of_memory
M=D

@i
M=0

(StateCheck)
  @KBD
  D=M
  @WhitenLoop
  D;JEQ

(BlackenLoop)
  @i
  D=M
  @SCREEN
  A=A+D
  M=-1 // ワードのビットを全て1で埋めるため-1を入れる
  @i
  M=M+1
  @StateCheck
  0;JMP
(WhitenLoop)
  @i
  D=M
  @SCREEN
  A=A+D
  M=0
  @i
  D=M
  @StateCheck
  D;JEQ
  @i
  M=M-1
  @StateCheck
  0;JMP

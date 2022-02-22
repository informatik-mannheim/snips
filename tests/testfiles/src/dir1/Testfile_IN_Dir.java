public class Foo {
  public static void main(String[] args) {
    // +IN Dir1
    int a = 1;
    // -IN Dir1
    System.out.println("Value is " + a);
  }
}
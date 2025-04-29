Feature: Arithmetic Operations

  Scenario: Adding two numbers
    Given I have two numbers 3 and 5
    When I add them
    Then I should get 8

  Scenario: Subtracting two numbers
    Given I have two numbers 5 and 3
    When I subtract them
    Then I should get 2

  Scenario: Multiplying two numbers
    Given I have two numbers 3 and 5
    When I multiply them
    Then I should get 15

  Scenario: Dividing two numbers
    Given I have two numbers 10 and 2
    When I divide them
    Then I should get 5

  Scenario: Dividing by zero
    Given I have two numbers 10 and 0
    When I divide them
    Then I should get an error "Division by zero"

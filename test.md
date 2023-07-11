# Marlowe Specification

## Version 3

#### Pablo Lamela Seijas Alexander Nemish David Smith

#### Simon Thompson Hernán Rajchert Brian Bush

#### December 31, 1979


## Contents



- 1 Marlowe
   - 1.1 Introduction
   - 1.2 The Marlowe Model.
      - 1.2.1 Data types.
      - 1.2.2 Quiescent
      - 1.2.3 Participants, accounts and state
      - 1.2.4 Core and Extended
   - 1.3 Specification generation and nomenclature
   - 1.4 Blockchain agnostic
- 2 Marlowe Core
   - 2.1 Types
      - 2.1.1 Participants, roles and addresses
      - 2.1.2 Multi-Asset token.
      - 2.1.3 Accounts
      - 2.1.4 Choices
      - 2.1.5 Values and Observations
      - 2.1.6 Actions and inputs
      - 2.1.7 Contracts
      - 2.1.8 State and Environment
   - 2.2 Semantics
      - 2.2.1 Compute Transaction.
      - 2.2.2 Fix Interval
      - 2.2.3 Apply All Inputs
      - 2.2.4 Reduce Contract Until Quiescent
      - 2.2.5 Reduction Loop
      - 2.2.6 Reduce Contract Step
      - 2.2.7 Apply Input
      - 2.2.8 Apply Cases
      - 2.2.9 Utilities
      - 2.2.10 Evaluate Value
      - 2.2.11 Evaluate Observation.
- 3 Marlowe Guarantees
   - 3.1 Money Preservation.
   - 3.2 Contracts Always Close.
   - 3.3 Positive Accounts
   - 3.4 Quiescent Result
   - 3.5 Reducing a Contract until Quiescence Is Idempotent
   - 3.6 Split Transactions Into Single Input Does Not Affect the Result
      - 3.6.1 Termination Proof
      - 3.6.2 All Contracts Have a Maximum Time.
      - 3.6.3 Contract Does Not Hold Funds After it Closes
      - 3.6.4 Transaction Bound
- A Contract examples
   - A.1 Simple Swap.
      - A.1.1 Contract definition
      - A.1.2 Example execution
      - A.1.3 Contract guarantees
- B ByteString
   - B.1 Ordering
- C Code exports
- D Marlowe Core JSON
   - D.1 Party.
   - D.2 Token
   - D.3 Payee
   - D.4 ChoicesId
   - D.5 Bound
   - D.6 Values
   - D.7 Observation
   - D.8 Action
   - D.9 Case
   - D.10 Contract
   - D.11 Input.
   - D.12 Transaction
   - D.13 Payment
   - D.14 State
   - D.15 TransactionWarning.
- D.16 IntervalError
- D.17 TransactionError
- D.18 TransactionOutput
- D.19 Full Contract Example
- D.20 Parse utils


# Chapter 1

# Marlowe

### 1.1 Introduction

Marlowe is a special purpose or domain-specific language (DSL) that is de-
signed to be usable by someone who is expert in the field of financial con-
tracts, somewhat lessening the need for programming skills.

Marlowe is modelled on special-purpose financial contract languages popu-
larised in the last decade or so by academics and enterprises such as LexiFi

(^1) , which provides contract software in the financial sector. In developing
Marlowe, we have adapted these languages to work on any blockchain §1.4.
Where we differ from non-blockchain approaches is in how we make sure
that the contract is followed. In the smart contracts world there is a saying
“Code is law”, which implies that the assets deposited in a contract will
follow its logic, without the ability of a human to change the rules. This
applies for both the intended and not intended behaviour (in the form of
bugs or exploits).
To reduce the probability of not intended behaviour, the Marlowe DSL is
designed with simplicity in mind. Without loops, recursion, or other features
that general purposes smart-contract languages (E.g: Plutus, Solidity) have,
it is easier to make certain claims. Each Marlowe contract can be reasoned
with a static analizer to avoid common pitfalls such as trying to Pay more
money than the available. And the _executable semantics_ that dictates the
logic of **all** Marlowe contracts is formalized with the proof-assistant Isabelle.
Chapter §1 provides an overview of the Marlowe language. Chapter §2 defines
the Core language and semantics in detail. Chapter §3 presents proofs that
(^1) https://www.lexifi.com/


guarantee that Marlowe contracts possess properties desirable for financial
agreements.

### 1.2 The Marlowe Model.

Marlowe _Contract_ s describe a series of steps, typically by describing the first
step, together with another (sub-)contract that describes what to do next.
For example, the contract _Pay a p t v c_ says “make a payment of _v_ number
of tokens _t_ to the party _p_ from the account _a_ , and then follow the contract
_c_ ”. We call _c_ the continuation of the contract. All paths of the contract are
made explicit this way, and each _Contract_ term is executed at most once.

#### 1.2.1 Data types.

The _Value_ s and _Observation_ s § 2.1.5only works with integers and booleans
respectively. There are no custom data types, records, tuples, nor string ma-
nipulation. There are also no floating point numbers, so in order to represent
currencies it is recommended to work with cents. Dates are only used in the
context of _Timeout_ s and they are absolute, but it is likely we’ll add relative
times in a future version.

#### 1.2.2 Quiescent

The blockchain can’t force a participant to make a transaction. To avoid
having a participant blocking the execution of a contract, whenever an _Input_
is expected, there is a _Timeout_ with a contingency continuation. For each
step, we can know in advance how long it can last, and we can extend this to
know the maximum duration and the amount of transactions of a contract.

#### 1.2.3 Participants, accounts and state

Once we define a contract, we can see how many participants it will have.
The number of participants is fixed for the duration of the contract, but there
are mechanisms to trade participation §2.1.1.

Each participant has an internal account that allows the contract to define
default owner for assets §2.1.3. Whenever a _Party_ deposits an asset in the
contract, they need to decide the default owner of that asset. Payments can
be made to transfer the default owner or to take the asset out of the contract.


If the contract is closed, the default owner can redeem the assets available in
their internal accounts.

The accounts, choices, and variables stored in the _State_ §2.1.8are global to
that contract.

#### 1.2.4 Core and Extended

The set of types and functions that conform the semantics executed in the
blockchain is called _Marlowe Core_ , and it’s formalized in chapter §2. To
improve usability, there is another set of types and functions that compile to
core, and it is called _Marlowe Extended_.

In the first version of the extended language, the only modification to the
DSL is the addition of template parameters. These allows an initial form
of contract reutilization, allowing to instantiate the same contract with dif-
ferent _Value_ s and _Timeout_ s. For the moment, the extended language is not
formalized in this specification but it will be added in the future

### 1.3 Specification generation and nomenclature

The Marlowe specification is formalized using the proof assistant Isabelle^2.
The code is written in a literate programming style and this document is
generated from the proofs. This improves code documentation and lowers
the probability of stale information.

As a drawback, the code/doc organization is more rigid. Isabelle require us to
define code in a bottom-up approach, having to define first the dependencies
and later the most complex structures.

The notation is closer to a Mathematical formula than a functional program-
ming language. There are some configurations in the _SpecificationLatexSugar_
theory file that makes the output be closer to code.

### 1.4 Blockchain agnostic

Marlowe is currently implemented on the Cardano Blockchain, but it is de-
signed to be Blockchain agnostic.

(^2) https://isabelle.in.tum.de/


Programs written in languages like Java and Python can be run on differ-
ent architectures, like amd64 or arm64, because they have interpreters and
runtimes for them. In the same way, the Marlowe interpreter could be im-
plemented to run on other blockchains, like Ethereum, Solana for example.

We make the following assumptions on the underlying Blockchain that allow
Marlowe Semantics to serve as a common abstraction:

In order to define the different _Token_ s that are used as currency in the
participants accounts §2.1.3, deposits, and payments, we need to be able to
express a _TokenName_ and _CurrencySymbol_.

**type-synonym** _TokenName_ = _ByteString_
**type-synonym** _CurrencySymbol_ = _ByteString_

To define a fixed participant in the contract §2.1.1and to make payouts to
them, we need to express an _Address_.

**type-synonym** _Address_ = _ByteString_

In the context of this specification, these _ByteString_ types are opaque, and we
don’t enforce a particular encoding or format, only that they can be sorted
§B.

The _Timeout_ s that prevent us from waiting forever for external _Input_ s are
represented by the number of milliseconds from the Unix Epoch^3.

**type-synonym** _POSIXTime_ = _int_

**type-synonym** _Timeout_ = _POSIXTime_

The _TimeInterval_ that defines the validity of a transaction is a tuple of
exclusive start and end time.

**type-synonym** _TimeInterval_ = _POSIXTime_ × _POSIXTime_

(^3) January 1st, 1970 at 00:00:00 UTC


# Chapter 2

# Marlowe Core

### 2.1 Types

This section introduces the data types of _Marlowe Core_ , which are composed
by the Marlowe DSL and also the types required to compute a _Transaction_.

Because of the literate programming nature of Isabelle §1.3, the types are
defined bottom-up. To follow just the DSL, a reader can start by looking at
a _Contract_ definition §2.1.7.

#### 2.1.1 Participants, roles and addresses

We should separate the notions of participant, role, and address in a Marlowe
contract. A participant (or _Party_ ) in the contract can be represented by
either a fixed _Address_ or a _Role_.

**type-synonym** _RoleName_ = _ByteString_

**datatype** _Party_ =
_Address Address_
| _Role RoleName_

An address party is defined by a Blockhain specific _Address_ §1.4 and it cannot
be traded (it is fixed for the lifetime of a contract).

A _Role_ , on the other hand, allows the participation of the contract to be
dynamic. Any user that can prove to have permission to act as _RoleName_
is able to carry out the actions assigned §2.1.6, and redeem the payments
issued to that role. The roles could be implemented as tokens^1 that can be

(^1) In the Cardano implementation roles are represented by native tokens and they are


traded. By minting multiple tokens for a particular role, several people can
be given permission to act on behalf of that role simultaneously, this allows
for more complex use cases.

#### 2.1.2 Multi-Asset token.

Inspired by Cardano’s Multi-Asset tokens^2 , Marlowe also supports to trans-
act with different assets. A _Token_ consists of a _CurrencySymbol_ that repre-
sents the monetary policy of the _Token_ and a _TokenName_ which allows to
have multiple tokens with the same monetary policy.

**datatype** _Token_ = _Token CurrencySymbol TokenName_

The Marlowe semantics treats both types as opaque _ByteString_.

#### 2.1.3 Accounts

The Marlowe model allows for a contract to store assets. All participants of
the contract implicitly own an account identified with an _AccountId_.

**type-synonym** _AccountId_ = _Party_

All assets stored in the contract must be in an internal account for one of
the parties; this way, when the contract is closed, all remaining assets can
be redeemed by their respective owners. These accounts are local: they only
exist (and are accessible) within the contract.

**type-synonym** _Accounts_ =(( _AccountId_ × _Token_ )× _int_ ) _list_

During its execution, the contract can invite parties to deposit assets into an
internal account through the construct “ _When_ [ _Deposit accountId party token
value_ ] _timeout continuation_ ". The contract can transfer assets internally
(between accounts) or externally (from an account to a party) by using the
term " _Pay accountId payee token value continuation_ ”, where _Payee_ is:

**datatype** _Payee_ = _Account AccountId_
| _Party Party_

A _Pay_ always takes money from an internal _AccountId_ , and the _Payee_ defines
if we transfer internally ( _Account p_ ) or externally ( _Party p_ )

distributed to addresses at the time a contract is deployed to the blockchain

(^2) https://docs.cardano.org/native-tokens/learn


#### 2.1.4 Choices

Choices – of integers – are identified by _ChoiceId_ which is defined with a
canonical name and the _Party_ who had made the choice:

**type-synonym** _ChoiceName_ = _ByteString_
**datatype** _ChoiceId_ = _ChoiceId ChoiceName Party_

Choices are _Bound_ ed. As an argument for the _Choice_ action §2.1.6, we pass
a list of _Bound_ s that limit the integer that we can choose. The _Bound_ data
type is a tuple of integers that represents an **inclusive** lower and upper
bound.

**datatype** _Bound_ = _Bound int int_

#### 2.1.5 Values and Observations

We can store a _Value_ in the Marlowe State §2.1.8using the _Let_ construct
§2.1.7, and we use a _ValueId_ to referrence it

**datatype** _ValueId_ = _ValueId ByteString_

_Value_ s and _Observation_ s are language terms that interact with most of the
other constructs. _Value_ evaluates to an integer and _Observation_ evaluates to
a boolean using _evalValue_ §2.2.10and _evalObservation_ §2.2.11respectively.

They are defined in a mutually recursive way as follows:

**datatype** _Value_ = _AvailableMoney AccountId Token_
| _Constant int_
| _NegValue Value_
| _AddValue Value Value_
| _SubValue Value Value_
| _MulValue Value Value_
| _DivValue Value Value_
| _ChoiceValue ChoiceId_
| _TimeIntervalStart_
| _TimeIntervalEnd_
| _UseValue ValueId_
| _Cond Observation Value Value_
**and** _Observation_ = _AndObs Observation Observation_
| _OrObs Observation Observation_
| _NotObs Observation_
| _ChoseSomething ChoiceId_
| _ValueGE Value Value_
| _ValueGT Value Value_


```
| ValueLT Value Value
| ValueLE Value Value
| ValueEQ Value Value
| TrueObs
| FalseObs
```
Three of the _Value_ terms look up information in the Marlowe state: _Avail-
ableMoney p t_ reports the amount of token _t_ in the internal account of party
_p_ ; _ChoiceValue i_ reports the most recent value chosen for choice _i_ , or zero if
no such choice has been made; and _UseValue i_ reports the most recent value
of the variable _i_ , or zero if that variable has not yet been set to a value.

_Constant v_ evaluates to the integer _v_ , while _NegValue x_ , _AddValue x y_ , _Sub-
Value x y_ , _MulValue x y_ , and _DivValue x y_ provide the common arithmetic
operations− _x_ , _x_ + _y_ , _x_ − _y_ , _x_ ∗ _y_ , and _x_ / _y_ , where division always rounds
(truncates) its result towards zero.

_Cond b x y_ represents a condition expression that evaluates to _x_ if _b_ is true
and to _y_ otherwise.

The last _Value_ s, _TimeIntervalStart_ and _TimeIntervalEnd_ , evaluate respec-
tively to the start or end of the validity interval for the Marlowe transaction.

For the observations, the _ChoseSomething i_ term reports whether a choice _i_
has been made thus far in the contract.

The terms _TrueObs_ and _FalseObs_ provide the logical constants _true_ and _false_.
The logical operators¬ _x_ , _x_ ∧ _y_ , and _x_ ∨ _y_ are represented by the terms
_NotObs x_ , _AndObs x y_ , and _OrObs x y_ , respectively.

Value comparisons _x_ < _y_ , _x_ ≤ _y_ , _x_ > _y_ , _x_ ≥ _y_ , and _x_ = _y_ are represented
by _ValueLT x y_ , _ValueLE x y_ , _ValueGT x y_ , _ValueGE x y_ , and _ValueEQ x y_.

#### 2.1.6 Actions and inputs

_Action_ s and _Input_ s are closely related. An _Action_ can be added in a list of
_Case_ s § 2.1.7as a way to declare the possible external _Input_ s a _Party_ can
include in a _Transaction_ at a certain time.

The different types of actions are:

**datatype** _Action_ = _Deposit AccountId Party Token Value_
| _Choice ChoiceId Bound list_
| _Notify Observation_


A _Deposit a p t v_ makes a deposit of # _v Token_ s _t_ from _Party p_ into account
_a_.

A choice _Choice i bs_ is made for a particular choice identified by the ChoiceId
§2.1.4 _i_ with a list of inclusive bounds _bs_ on the values that are acceptable.
For example,[ _Bound 0 0_ , _Bound 3 5_ ]offers the choice of one of 0, 3, 4 and
5.

A notification can be triggered by anyone as long as the _Observation_ evaluates
to _true_. If multiple _Notify_ are present in the _Case_ list, the first one with a
_true_ observation is matched.

For each _Action_ , there is a corresponding _Input_ that can be included inside
a _Transaction_

**type-synonym** _ChosenNum_ = _int_

**datatype** _Input_ = _IDeposit AccountId Party Token int_
| _IChoice ChoiceId ChosenNum_
| _INotify_

The differences between them are:

- _Deposit_ uses a _Value_ while _IDeposit_ has the _int_ it was evaluated to
    with _evalValue_ §2.2.10.
- _Choice_ defines a list of valid _Bound_ s while _IChoice_ has the actual _Cho-_
    _senNum_.
- _Notify_ has an _Observation_ while _INotify_ does not have arguments, the
    _Observation_ must evaluate to true inside the _Transaction_

#### 2.1.7 Contracts

Marlowe is a continuation-based language, this means that a _Contract_ can
either be a _Close_ or another construct that recursively has a _Contract_. Even-
tually, **all** contracts end up with a _Close_ construct.

_Case_ and _Contract_ are defined in a mutually recursive way as follows:

**datatype** _Case_ = _Case Action Contract_
**and** _Contract_ = _Close_
| _Pay AccountId Payee Token Value Contract_
| _If Observation Contract Contract_


```
| When Case list Timeout Contract
| Let ValueId Value Contract
| Assert Observation Contract
```
_Close_ is the simplest contract, when we evaluate it, the execution is completed
and we generate _Payment_ s § **??** for the assets in the internal accounts to their
default owners^3.

The contract _Pay a p t v c_ , generates a _Payment_ from the internal account _a_
to a payee §2.1.3 _p_ of # _v Token_ s and then continues to contract _c_. Warnings
will be generated if the value _v_ is not positive, or if there is not enough in the
account to make the payment in full. In the latter case, a partial payment
(of the available amount) is made

The contract _If obs x y_ allows branching. We continue to branch _x_ if the
_Observation obs_ evaluates to _true_ , or to branch _y_ otherwise.

_When_ is the most complex constructor for contracts, with the form _When cs
t c_. The list _cs_ contains zero or more pairs of _Action_ s and _Contract_ continu-
ations. When we do a _computeTransaction_ §2.2.1, we follow the continuation
associated to the first _Action_ that matches the _Input_. If no action is matched
it returns a _ApplyAllNoMatchError_. If a valid _Transaction_ is computed with
a _TimeInterval_ with a start time bigger than the _Timeout t_ , the contingency
continuation _c_ is evaluated. The explicit timeout mechanism is what allows
Marlowe to avoid waiting forever for external inputs.

A _Let_ contract _Let i v c_ allows a contract to record a value using an identifier
_i_. In this case, the expression _v_ is evaluated, and the result is stored with
the name _i_. The contract then continues as _c_. As well as allowing us to
use abbreviations, this mechanism also means that we can capture and save
volatile values that might be changing with time, e.g. the current price of oil,
or the current time, at a particular point in the execution of the contract, to
be used later on in contract execution.

An assertion contract _Assert b c_ does not have any effect on the state of
the contract, it immediately continues as _c_ , but it issues a warning if the
observation _b_ evaluates to false. It can be used to ensure that a property
holds in a given point of the contract, since static analysis will fail if any
execution causes a warning. The _Assert_ term might be removed from future
on-chain versions of Marlowe.

(^3) Even if the payments are generated one at a time (per account and per Token), the
cardano implementation generates a single transaction


#### 2.1.8 State and Environment

The internal state of a Marlowe contract consists of the current balances in
each party’s account, a record of the most recent value of each type of choice,
a record of the most recent value of each variable, and the lower bound for the
current time that is used to refine time intervals and ensure _TimeIntervalStart_
never decreases. The data for accounts, choices, and bound values are stored
as association lists.

**record** _State_ = _accounts_ :: _Accounts
choices_ ::( _ChoiceId_ × _ChosenNum_ ) _list
boundValues_ ::( _ValueId_ × _int_ ) _list
minTime_ :: _POSIXTime_

The execution environment of a Marlowe contract simply consists of the
(inclusive) time interval within which the transaction is occurring.

**record** _Environment_ = _timeInterval_ :: _TimeInterval_

— TODO: see if we want to add data types of Semantic here (Transaction, etc)
or if we want to move this types to Semantic

**datatype** _IntervalError_ = _InvalidInterval TimeInterval_
| _IntervalInPastError POSIXTime TimeInterval_

**datatype** _IntervalResult_ = _IntervalTrimmed Environment State_
| _IntervalError IntervalError_

### 2.2 Semantics

Marlowe’s behavior is defined via the _operational semantics_ (or _executable
semantics_ ) of the Isabelle implementation of its _computeTransaction_ func-
tion. That function calls several auxiliary functions to apply inputs and find
a quiescent state of the contract. These, in turn, call evaluators for _Value_
and _Observation_.


#### 2.2.1 Compute Transaction.

The entry point into Marlowe semantics is the function _computeTransaction_
that applies input to a prior state to transition to a posterior state, perhaps
reporting warnings or throwing an error, all in the context of an environment
for the transaction.

_computeTransaction_ :: _Transaction_ ⇒ _State_ ⇒ _Contract_ ⇒ _TransactionOut-
put_

FIXME: Print record: _Transaction_

**datatype** _TransactionOutput_ =
_TransactionOutput
TransactionOutputRecord_
| _TransactionError TransactionError_

FIXME: Print record: _TransactionOutputRecord_

This function adjusts the time interval for the transaction using _fixInter-
val_ and then applies all of the transaction inputs to the contract using _ap-
plyAllInputs_. It reports relevant warnings and throws relevant errors.

computeTransaction ::
Transaction_ext () -> State_ext () -> Contract -> TransactionOutput;
computeTransaction tx state contract =
let {
inps = inputs tx;
} in (case fixInterval (interval tx) state of {
IntervalTrimmed env fixSta ->
(case applyAllInputs env fixSta contract inps of {
ApplyAllSuccess reduced warnings payments newState cont
->
(if not reduced &&
(not (equal_Contract contract Close) ||
null (accounts state))
then TransactionError TEUselessTransaction
else TransactionOutput
(TransactionOutputRecord_ext warnings payments
newState
cont ()));
ApplyAllNoMatchError -> TransactionError TEApplyNoMatchError;
ApplyAllAmbiguousTimeIntervalError ->


```
TransactionError TEAmbiguousTimeIntervalError;
});
IntervalError errora -> TransactionError (TEIntervalError errora);
});
```
#### 2.2.2 Fix Interval

The _fixInterval_ functions combines the minimum-time constraint of _State_
with the time interval of _Environment_ to yield a “trimmed” validity interval
and a minimum time for the new state that will result from applying the
transaction. It throws an error if the interval is nonsensical or in the past.

FIXME: print type synonym: _IntervalResult_

fixInterval :: (Int, Int) -> State_ext () -> IntervalResult;
fixInterval (low, high) state =
let {
curMinTime = minTime state;
newLow = max low curMinTime;
curInterval = (newLow, high);
env = Environment_ext curInterval ();
newState = minTime_update (\ _ -> newLow) state;
} in (if less_int high low then IntervalError (InvalidInterval (low,
high))
else (if less_int high curMinTime
then IntervalError (IntervalInPastError curMinTime (low,
high))
else IntervalTrimmed env newState));

#### 2.2.3 Apply All Inputs

The _applyAllInputs_ function iteratively progresses the contract and applies
the transaction inputs to the state, checking for errors along the way and con-
tinuing until all the inputs are consumed and the contract reaches a quiescent
state.

applyAllInputs ::
Environment_ext () -> State_ext () -> Contract -> [Input] -> ApplyAllResult;
applyAllInputs env state contract inputs =
applyAllLoop False env state contract inputs [] [];


applyAllLoop ::
Bool ->
Environment_ext () ->
State_ext () ->
Contract ->
[Input] -> [TransactionWarning] -> [Payment] -> ApplyAllResult;
applyAllLoop contractChanged env state contract inputs warnings payments
=
(case reduceContractUntilQuiescent env state contract of {
ContractQuiescent reduced reduceWarns pays curState cont ->
(case inputs of {
[] -> ApplyAllSuccess (contractChanged || reduced)
(warnings ++ convertReduceWarnings reduceWarns)
(payments ++ pays) curState cont;
input : rest ->
(case applyInput env curState input cont of {
Applied applyWarn newState conta ->
applyAllLoop True env newState conta rest
(warnings ++
convertReduceWarnings reduceWarns ++
convertApplyWarning applyWarn)
(payments ++ pays);
ApplyNoMatchError -> ApplyAllNoMatchError;
});
});
RRAmbiguousTimeIntervalError -> ApplyAllAmbiguousTimeIntervalError;
});

#### 2.2.4 Reduce Contract Until Quiescent

The _reduceContractUntilQuiescent_ executes as many non-input steps of the
contract as is possible. Marlowe semantics do not allow partial execution of
a series of non-input steps.

reduceContractUntilQuiescent ::
Environment_ext () -> State_ext () -> Contract -> ReduceResult;
reduceContractUntilQuiescent env state contract =
reductionLoop False env state contract [] [];


#### 2.2.5 Reduction Loop

The _reductionLoop_ function attempts to apply the next, non-input step to
the contract. It emits warnings along the way and it will through an error if
it encounters an ambiguous time interval.

reductionLoop ::
Bool ->
Environment_ext () ->
State_ext () -> Contract -> [ReduceWarning] -> [Payment] -> ReduceResult;
reductionLoop reduced env state contract warnings payments =
(case reduceContractStep env state contract of {
Reduced warning effect newState ncontract ->
let {
newWarnings =
(if equal_ReduceWarning warning ReduceNoWarning then warnings
else warning : warnings);
a = (case effect of {
ReduceNoPayment -> payments;
ReduceWithPayment payment -> payment : payments;
});
} in reductionLoop True env newState ncontract newWarnings a;
NotReduced ->
ContractQuiescent reduced (reverse warnings) (reverse payments)
state
contract;
AmbiguousTimeIntervalReductionError -> RRAmbiguousTimeIntervalError;
});

#### 2.2.6 Reduce Contract Step

The _reduceContractStep_ function handles the progression of the _Contract_
in the absence of inputs: it performs the relevant action (payments, state-
change, etc.), reports warnings, and throws errors if needed. It stops reducing
the contract at the point when the contract requires external input.

Note that this function should report an implicit payment of zero (due to
lack of funds) as a partial payment of zero, not as a non-positive payment.
An explicit payment of zero (due to the contract actually specifying a zero
payment) should be reported as a non-positive payment.


reduceContractStep ::
Environment_ext () -> State_ext () -> Contract -> ReduceStepResult;
reduceContractStep uu state Close =
(case refundOne (accounts state) of {
Nothing -> NotReduced;
Just ((party, (token, money)), newAccount) ->
let {
newState = accounts_update (\ _ -> newAccount) state;
} in Reduced ReduceNoWarning
(ReduceWithPayment (Payment party (Party party) token money))
newState Close;
});
reduceContractStep env state (Pay accId payee token val cont) =
let {
moneyToPay = evalValue env state val;
} in (if less_eq_int moneyToPay Zero_int
then let {
warning = ReduceNonPositivePay accId payee token moneyToPay;
} in Reduced warning ReduceNoPayment state cont
else let {
balance = moneyInAccount accId token (accounts state);
paidMoney = min balance moneyToPay;
newBalance = minus_int balance paidMoney;
newAccs =
updateMoneyInAccount accId token newBalance (accounts
state);
warning =
(if less_int paidMoney moneyToPay
then ReducePartialPay accId payee token paidMoney
moneyToPay
else ReduceNoWarning);
} in (case giveMoney accId payee token paidMoney newAccs
of {
(payment, finalAccs) ->
Reduced warning payment
(accounts_update (\ _ -> finalAccs) state) cont;
}));
reduceContractStep env state (If obs cont1 cont2) =
let {
a = (if evalObservation env state obs then cont1 else cont2);
} in Reduced ReduceNoWarning ReduceNoPayment state a;
reduceContractStep env state (When uv timeout cont) =
(case timeInterval env of {


(startTime, endTime) ->
(if less_int endTime timeout then NotReduced
else (if less_eq_int timeout startTime
then Reduced ReduceNoWarning ReduceNoPayment state cont
else AmbiguousTimeIntervalReductionError));
});
reduceContractStep env state (Let valId val cont) =
let {
evaluatedValue = evalValue env state val;
boundVals = boundValues state;
newState =
boundValues_update (\ _ -> insert valId evaluatedValue boundVals)
state;
warn = (case lookup valId boundVals of {
Nothing -> ReduceNoWarning;
Just oldVal -> ReduceShadowing valId oldVal evaluatedValue;
});
} in Reduced warn ReduceNoPayment newState cont;
reduceContractStep env state (Assert obs cont) =
let {
warning =
(if evalObservation env state obs then ReduceNoWarning
else ReduceAssertionFailed);
} in Reduced warning ReduceNoPayment state cont;

#### 2.2.7 Apply Input

The _applyInput_ function attempts to apply the next input to each _Case_ in
the _When_ , in sequence.

applyInput ::
Environment_ext () -> State_ext () -> Input -> Contract -> ApplyResult;
applyInput env state input (When cases t cont) =
applyCases env state input cases;
applyInput env state input Close = ApplyNoMatchError;
applyInput env state input (Pay v va vb vc vd) = ApplyNoMatchError;
applyInput env state input (If v va vb) = ApplyNoMatchError;
applyInput env state input (Let v va vb) = ApplyNoMatchError;
applyInput env state input (Assert v va) = ApplyNoMatchError;


#### 2.2.8 Apply Cases

The _applyCases_ function attempts to match an _Input_ to an _Action_ , compute
the new contract state, emit warnings, throw errors if needed, and determine
the appropriate continuation of the contract.

applyCases ::
Environment_ext () -> State_ext () -> Input -> [Case] -> ApplyResult;
applyCases env state (IDeposit accId1 party1 tok1 amount)
(Case (Deposit accId2 party2 tok2 val) cont : rest) =
(if equal_Party accId1 accId2 &&
equal_Party party1 party2 &&
equal_Token tok1 tok2 && equal_int amount (evalValue env state
val)
then let {
warning =
(if less_int Zero_int amount then ApplyNoWarning
else ApplyNonPositiveDeposit party1 accId2 tok2 amount);
newState =
accounts_update
(\ _ -> addMoneyToAccount accId1 tok1 amount (accounts
state))
state;
} in Applied warning newState cont
else applyCases env state (IDeposit accId1 party1 tok1 amount) rest);
applyCases env state (IChoice choId1 choice)
(Case (Choice choId2 bounds) cont : rest) =
(if equal_ChoiceId choId1 choId2 && inBounds choice bounds
then let {
newState =
choices_update (\ _ -> insert choId1 choice (choices state))
state;
} in Applied ApplyNoWarning newState cont
else applyCases env state (IChoice choId1 choice) rest);
applyCases env state INotify (Case (Notify obs) cont : rest) =
(if evalObservation env state obs then Applied ApplyNoWarning state
cont
else applyCases env state INotify rest);
applyCases env state (IDeposit accId1 party1 tok1 amount)
(Case (Choice vb vc) va : rest) =
applyCases env state (IDeposit accId1 party1 tok1 amount) rest;
applyCases env state (IDeposit accId1 party1 tok1 amount)
(Case (Notify vb) va : rest) =


applyCases env state (IDeposit accId1 party1 tok1 amount) rest;
applyCases env state (IChoice choId1 choice)
(Case (Deposit vb vc vd ve) va : rest) =
applyCases env state (IChoice choId1 choice) rest;
applyCases env state (IChoice choId1 choice) (Case (Notify vb) va : rest)
=
applyCases env state (IChoice choId1 choice) rest;
applyCases env state INotify (Case (Deposit vb vc vd ve) va : rest) =
applyCases env state INotify rest;
applyCases env state INotify (Case (Choice vb vc) va : rest) =
applyCases env state INotify rest;
applyCases env state acc [] = ApplyNoMatchError;

#### 2.2.9 Utilities

The _moneyInAccount_ , _updateMoneyInAccount_ , and _addMoneyToAccount_ func-
tions read, write, and increment the funds in a particular account of the _State_ ,
respectively. The _giveMoney_ function transfer funds internally between ac-
counts. The _refundOne_ function finds the first account with funds in it.

moneyInAccount :: Party -> Token -> [((Party, Token), Int)] -> Int;
moneyInAccount accId token accountsV =
findWithDefault Zero_int (accId, token) accountsV;

updateMoneyInAccount ::
Party -> Token -> Int -> [((Party, Token), Int)] -> [((Party, Token),
Int)];
updateMoneyInAccount accId token money accountsV =
(if less_eq_int money Zero_int then delete (accId, token) accountsV
else insert (accId, token) money accountsV);

addMoneyToAccount ::
Party -> Token -> Int -> [((Party, Token), Int)] -> [((Party, Token),
Int)];
addMoneyToAccount accId token money accountsV =
let {
balance = moneyInAccount accId token accountsV;
newBalance = plus_int balance money;
} in (if less_eq_int money Zero_int then accountsV
else updateMoneyInAccount accId token newBalance accountsV);


giveMoney ::
Party ->
Payee ->
Token ->
Int ->
[((Party, Token), Int)] -> (ReduceEffect, [((Party, Token),
Int)]);
giveMoney accountId payee token money accountsV =
let {
a = (case payee of {
Account accId -> addMoneyToAccount accId token money accountsV;
Party _ -> accountsV;
});
} in (ReduceWithPayment (Payment accountId payee token money), a);

refundOne ::
[((Party, Token), Int)] ->
Maybe ((Party, (Token, Int)), [((Party, Token), Int)]);
refundOne (((accId, tok), money) : rest) =
(if less_int Zero_int money then Just ((accId, (tok, money)), rest)
else refundOne rest);
refundOne [] = Nothing;

#### 2.2.10 Evaluate Value

Given the _Environment_ and the current _State_ , the _evalValue_ function evalu-
ates a _Value_ into a number

_evalValue_ :: _Environment_ ⇒ _State_ ⇒ _Value_ ⇒ _int_

**Available Money**

For the _AvailableMoney_ case, _evalValue_ will give us the amount of _Token_ s
that a _Party_ has in their internal account.

_evalValue env state_ ( _AvailableMoney accId token_ ) = _findWithDefault 0_ ( _accId_ ,
_token_ ) ( _accounts state_ )


**Constant**

For the _Constant_ case, _evalValue_ will always evaluate to the same value

_evalValue env state_ ( _Constant integer_ ) = _integer_

**Addition**

For the _AddValue_ case, _evalValue_ will evaluate both sides and add them
together.

_evalValue env state_ ( _AddValue lhs rhs_ ) = _evalValue env state lhs_ + _evalValue
env state rhs_

Addition is associative and commutative:

_evalValue env sta_ ( _AddValue x_ ( _AddValue y z_ )) = _evalValue env sta_ ( _AddValue_
( _AddValue x y_ ) _z_ )

_evalValue env sta_ ( _AddValue x y_ ) = _evalValue env sta_ ( _AddValue y x_ )

**Subtraction**

For the _SubValue_ case, _evalValue_ will evaluate both sides and subtract the
second value from the first.

_evalValue env state_ ( _SubValue lhs rhs_ ) = _evalValue env state lhs_ − _evalValue
env state rhs_

**Negation**

For every value _x_ there is the complement _NegValue x_ so that

_evalValue env sta_ ( _AddValue x_ ( _NegValue x_ )) = _0_

**Multiplication**

For the _MulValue_ case, _evalValue_ will evaluate both sides and multiply them.

_evalValue env state_ ( _MulValue lhs rhs_ ) = _evalValue env state lhs_ ∗ _evalValue
env state rhs_


**Division**

Division is a special case because we only evaluate to natural numbers:

- If the denominator is 0, the result is also 0. Other languages uses NaN
    or Infinity to represent this case
- The result will be rounded towards zero.

_evalValue env state_ ( _DivValue lhs rhs_ )=
( _letn_ = _evalValue env state lhs_ ;
_d_ = _evalValue env state rhs
in ifd_ = _0 then 0 elsen quot d_ )

TODO: lemmas around division? maybe extend the following to proof eval-
Value and not just div

_c_ 6 = _0_ =⇒ _c_ ∗ _a div_ ( _c_ ∗ _b_ ) = _a div b_

_c_ 6 = _0_ =⇒ | _c_ ∗ _a_ | _div_ | _c_ ∗ _b_ |=| _a_ | _div_ | _b_ |

COMMENT(BWB): I suggest that the lemmas be (i) exact multiples di-
vide with no remainder, (ii) the remainder equals the excess above an exact
multiple, and (iii) negation commutues with division.

**Choice Value**

For the _ChoiceValue_ case, _evalValue_ will look in its state if a _Party_ has made
a choice for the _ChoiceName_. It will default to zero if it doesn’t find it.

_evalValue env state_ ( _ChoiceValue choId_ ) = _findWithDefault 0 choId_ ( _choices
state_ )

**Time Interval Start**

All transactions are executed in the context of a valid time interval. For the
_TimeIntervalStart_ case, _evalValue_ will return the beginning of that interval.

_evalValue env state TimeIntervalStart_ = _fst_ ( _timeInterval env_ )


**Time Interval End**

All transactions are executed in the context of a valid time interval. For the
_TimeIntervalEnd_ case, _evalValue_ will return the end of that interval.

_evalValue env state TimeIntervalEnd_ = _snd_ ( _timeInterval env_ )

**Use Value**

For the _TimeIntervalEnd_ case, _evalValue_ will look in its state for a bound
_ValueId_. It will default to zero if it doesn’t find it.

_evalValue env state_ ( _UseValue valId_ ) = _findWithDefault 0 valId_ ( _boundValues
state_ )

**Conditional Value**

For the _Cond_ case, _evalValue_ will first call _evalObservation_ on the condition,
and it will evaluate the the true or false value depending on the result.

_evalValue env state_ ( _Cond cond thn els_ ) = ( _ifevalObservation env state cond
thenevalValue env state thnelseevalValue env state els_ )

#### 2.2.11 Evaluate Observation.

Given the _Environment_ and the current _State_ , the _evalObservation_ function
evaluates an _Observation_ into a number

_evalObservation_ :: _Environment_ ⇒ _State_ ⇒ _Observation_ ⇒ _bool_

**True and False**

The logical constants _true_ and _false_ are trivially evaluated.

_evalObservation env state TrueObs_ = _True_

_evalObservation env state FalseObs_ = _False_

**Not, And, Or**

The standard logical operators¬,∧, and∨are evaluated in a straightforward
manner.


_evalObservation env state_ ( _NotObs subObs_ ) = (¬ _evalObservation env state
subObs_ )

_evalObservation env state_ ( _AndObs lhs rhs_ ) = ( _evalObservation env state lhs_
∧ _evalObservation env state rhs_ )

_evalObservation env state_ ( _OrObs lhs rhs_ ) = ( _evalObservation env state lhs_
∨ _evalObservation env state rhs_ )

**Comparison of Values**

Five functions are provided for the comparison (equality and ordering of
integer values) have traditional evaluations:=,<,≤,>, and≥.

_evalObservation env state_ ( _ValueEQ lhs rhs_ ) = ( _evalValue env state lhs_ =
_evalValue env state rhs_ )

_evalObservation env state_ ( _ValueLT lhs rhs_ ) = ( _evalValue env state lhs_ <
_evalValue env state rhs_ )

_evalObservation env state_ ( _ValueLE lhs rhs_ ) = ( _evalValue env state lhs_ ≤
_evalValue env state rhs_ )

_evalObservation env state_ ( _ValueGT lhs rhs_ ) = ( _evalValue env state rhs_ <
_evalValue env state lhs_ )

_evalObservation env state_ ( _ValueGE lhs rhs_ ) = ( _evalValue env state rhs_ ≤
_evalValue env state lhs_ )

**Chose Something**

The _ChoseSometing i_ term evaluates to true if the a choice _i_ was previously
made in the history of the contract.

_evalObservation env state_ ( _ChoseSomething choId_ ) = _member choId_ ( _choices
state_ )


# Chapter 3

# Marlowe Guarantees

We can also use proof assistants to demonstrate that the Marlowe semantics
presents certain desirable properties, such as that money is preserved and
anything unspent is returned to users by the end of the execution of any
contract.

**Auxillary Functions**

Many of the proofs in this chapter rely on function _playTrace_ and _play-
TraceAux_ that execute a sequence of transactions using the Marlowe seman-
tics defined in _computeTransaction_. They also rely on starting from a valid
and positive contract state, _validAndPositive-state_ and a function _maxTime-
Contract_ that extracts the latest timeout from the contract.

_playTrace_ :: _int_ ⇒ _Contract_ ⇒ _Transaction list_ ⇒ _TransactionOutput_

_playTraceAux_ :: _TransactionOutputRecord_ ⇒ _Transaction list_ ⇒ _Transac-
tionOutput_

_validAndPositive-state_ :: _State_ ⇒ _bool_

_maxTimeContract_ :: _Contract_ ⇒ _int_

### 3.1 Money Preservation.

One of the dangers of using smart contracts is that a badly written one can
potentially lock its funds forever. By the end of the contract, all the money
paid to the contract must be distributed back, in some way, to a subset of
the participants of the contract. To ensure this is the case we proved two
properties: “Money Preservation” and “Contracts Always Close”.


Regarding money preservation, money is not created or destroyed by the
semantics. More specifically, the money that comes in plus the money in the
contract before the transaction must be equal to the money that comes out
plus the contract after the transaction, except in the case of an error.

_moneyInTransactions tra_ = _moneyInPlayTraceResult tra_ ( _playTrace sl con-
tract tra_ )

where _moneyInTransactions_ and _moneyInPlayTraceResult_ measure the funds
in the transactions applied to a contract versus the funds in the contract state
and the payments that it has made while executing.

### 3.2 Contracts Always Close.

For every Marlowe Contract there is a time after which an empty transaction
can be issued that will close the contract and refund all the money in its
accounts.

FIXME: This theorem doesn’t actually prove the narrative. Are we missing
a theorem?

[[ _validAndPositive-state sta_ ; _accounts sta_ 6 = []∨ _cont_ 6 = _Close_ ]] =⇒ ∃ _inp_.
_isClosedAndEmpty_ ( _computeTransaction inp sta cont_ )

### 3.3 Positive Accounts

There are some values for State that are allowed by its type but make no
sense, especially in the case of Isabelle semantics where we use lists instead
of maps:

1. The lists represent maps, so they should have no repeated keys.
2. We want two maps that are equal to be represented the same, so we
    force keys to be in ascending order.
3. We only want to record those accounts that contain a positive amount.

We call a value for State valid if the first two properties are true. And we
say it has positive accounts if the third property is true.

FIXME: Address the review comment "Is this a note for us or the explanation
to the user of what _playTraceAux-preserves-validAndPositive-state_ proves?".


[[ _validAndPositive-state_ ( _txOutState txIn_ ); _playTraceAux txIn transList_ = _Trans-
actionOutput txOut_ ]] =⇒ _validAndPositive-state_ ( _txOutState txOut_ )

### 3.4 Quiescent Result

A contract is quiescent if and only if the root construct is _When_ , or if the
contract is _Close_ and all accounts are empty. If an input _State_ is valid and
accounts are positive, then the output will be quiescent, _isQuiescent_.

The following always produce quiescent contracts:

- reductionLoop §2.2.5
- reduceContractUntilQuiescent §2.2.4
- applyAllInputs §2.2.3
- computeTransaction §2.2.1
- playTrace § 3

_playTrace sl cont_ ( _h_ : _t_ ) = _TransactionOutput traOut_ =⇒ _isQuiescent_ ( _txOutContract
traOut_ ) ( _txOutState traOut_ )

### 3.5 Reducing a Contract until Quiescence Is Idempotent

Once a contract is quiescent, further reduction will not change the contract
or state, and it will not produce any payments or warnings.

_reduceContractUntilQuiescent env state contract_ = _ContractQuiescent re-
ducedAfter wa pa nsta ncont_ =⇒ _reduceContractUntilQuiescent env nsta
ncont_ = _ContractQuiescent False_ [] [] _nsta ncont_

### 3.6 Split Transactions Into Single Input Does Not Affect the Result

Applying a list of inputs to a contract produces the same result as applying
each input singly.

_playTraceAux acc tral_ = _playTraceAux acc_ ( _traceListToSingleInput tral_ )


#### 3.6.1 Termination Proof

Isabelle automatically proves termination for most function. However, this is
not the case for _reductionLoop_ , but it is manually proved that the reduction
loop monotonically reduces the size of the contract (except for _Close_ , which
reduces the number of accounts), this is sufficient to prove termination.

_reduceContractStep env sta c_ = _Reduced twa tef nsta nc_ =⇒ _evalBound nsta
nc_ < _evalBound sta c_

#### 3.6.2 All Contracts Have a Maximum Time.

If one sends an empty transaction with time equal to _maxTimeContract_ , then
the contract will close.

```
validAndPositive-state sta
minTime sta ≤ iniTime maxTimeContract cont ≤ iniTime
iniTime ≤ endTime accounts sta 6 = []∨ cont 6 = Close
isClosedAndEmpty ( computeTransaction (| interval = ( iniTime , endTime ), inputs = []|) sta cont )
```
#### 3.6.3 Contract Does Not Hold Funds After it Closes

Funds are not held in a contract after it closes.

_computeTransaction tra sta Close_ = _TransactionOutput trec_ =⇒ _txOutWarn-
ings trec_ = []

#### 3.6.4 Transaction Bound

There is a maximum number of transaction that can be accepted by a con-
tract.

_playTrace sl c l_ = _TransactionOutput txOut_ =⇒ | _l_ | ≤ _maxTransactionsIni-
tialState c_


# Appendix A

# Contract examples

This appendix includes some example contracts embedded inside isabelle
with their corresponding guarantees:

### A.1 Simple Swap.

A simple swap contract consists on two parties exchanging some _amount_ of
_Token_ s atomically. Each participant needs to deposit their tokens into the
contract by a certain _depositDeadline_. If they do, the contract makes the
swap and pays the participants, if one of the participant fails to make the
deposit, the funds held by the contract can be redeemed by the owner.

#### A.1.1 Contract definition

To reduce the number of parameters we bundle the information required by
each participant into a record.

**record** _SwapParty_ =
— A participant of the contract,
_party_ :: _Party_
— wants to swap an _amount_ of _Token
amount_ :: _Value
currency_ :: _Token_
— before a deadline
_depositDeadline_ :: _Timeout_

The following helper function allows participants to deposit their tokens into
the contract.

**fun** _makeDeposit_ :: _SwapParty_ ⇒ _Contract_ ⇒ _Contract_ **where**


```
makeDeposit sp continue =
— The contract waits for a deposit
When
[
Case
( Deposit
— into the internal account of the party
( party sp )
— from the party wallet
( party sp )
— Amount of tokens
( currency sp )
( amount sp )
)
— Once the deposit has been made, execute the continuation
continue
]
— If the tokens haven’t been deposited by the deadline, close the contract.
— This will return all current funds to their owners.
( depositDeadline sp ) Close
```
The following helper function makes a _Payment_ from one party to the other

**fun** _makePayment_ :: _SwapParty_ ⇒ _SwapParty_ ⇒ _Contract_ ⇒ _Contract_ **where**
_makePayment src dest_ =
— The contract makes a Payment
_Pay_
— from the party internal account
( _party src_ )
— to the destination wallet
( _Party_ ( _party dest_ ))
— of the number of tokens from the source
( _currency src_ )( _amount src_ )

The actual swap contract waits for both parties to make their deposits, then
makes the payout and closes.

**fun** _swap_ :: _SwapParty_ ⇒ _SwapParty_ ⇒ _Contract_ **where**
_swap p1 p2_ = _makeDeposit p1_
( _makeDeposit p2_
( _makePayment p1 p2_
( _makePayment p2 p1 Close_
)))


#### A.1.2 Example execution

Let’s define two participants that want to trade USD and ADA in the cardano
blockchain.

**definition** _adaProvider_ = _Role_ ( _BS_ ′′ _Ada Provider_ ′′)
**definition** _dollarProvider_ = _Role_ ( _BS_ ′′ _Dollar Provider_ ′′)

In cardano, the ADA symbol is represented by the empty string

**definition** _adaToken_ = _Token_ ( _BS_ ′′′′)( _BS_ ′′′′)
**definition** _dollarToken_ = _Token_ ( _BS_ ′′ _85bb65_ ′′)( _BS_ ′′ _dollar_ ′′)

The contract can be created as follow.

**definition**
_swapExample_ =
_swap_
— Party A trades 10 lovelaces
— deposited before Monday, October 3, 2022 4:00:00 PM GMT
(| _party_ = _adaProvider_
, _amount_ = _Constant 10_
, _currency_ = _adaToken_
, _depositDeadline_ = _1664812800000_
|)
— Party B trades 20 cents
— deposited before Monday, October 3, 2022 5:00:00 PM GMT
(| _party_ = _dollarProvider_
, _amount_ = _Constant 20_
, _currency_ = _dollarToken_
, _depositDeadline_ = _1664816400000_
|)

**Happy path**

If both parties deposit before their deadline,

**definition**
_happyPathTransactions_ =
[
— First party deposit
(| _interval_ =( _1664812600000_ , _1664812700000_ )
, _inputs_ =[


```
IDeposit
adaProvider
adaProvider
adaToken
10
]
|)
— Second party deposit
,(| interval =( 1664812900000 , 1664813100000 )
, inputs =[
IDeposit
dollarProvider
dollarProvider
dollarToken
20
]
|)
]
```
payments are made to swap the tokens

**definition**
_happyPathPayments_ =
[ _Payment adaProvider_ ( _Party dollarProvider_ ) _adaToken 10_
, _Payment dollarProvider_ ( _Party adaProvider_ ) _dollarToken 20_
]

and the contract is closed without emitting a warning

**proposition**
_playTrace 0 swapExample happyPathTransactions_ = _TransactionOutput txOut_
=⇒
_txOutContract txOut_ = _Close_
∧ _txOutPayments txOut_ = _happyPathPayments_
∧ _txOutWarnings txOut_ =[]

#### A.1.3 Contract guarantees

**Number of transactions**

Counting the amount of When’s, it is easy to notice that there can be at
most two transactions

**proposition** _maxTransactionsInitialState_ ( _swap a b_ )= _2_


Expressed in a different way, if we use the lemma defined in §3.6.4we can state
that, if the execution of the contract yields a succesful _TransactionOutput_ ,
then the number of transactions must be lower or equal than _2_

**lemma**
_playTrace
initialTime_
( _swap a b_ )
_transactions_ = _TransactionOutput txOut_
=⇒ _length transactions_ ≤ _2_

**Maximum time**

If the deadline of the second party is bigger than the first, then that deadline
is the maximum time of the contract.

**proposition**
_sp1_ =
(| _party_ = _p1_
, _amount_ = _a1_
, _currency_ = _t1_
, _depositDeadline_ = _d1_
|)
=⇒ _sp2_ =
(| _party_ = _p2_
, _amount_ = _a2_
, _currency_ = _t2_
, _depositDeadline_ = _d2_
|)
=⇒ _d2_ > _d1_
=⇒ _d1_ > _0_
=⇒ _contract_ = _swap sp1 sp2_
=⇒ _maxTimeContract_ ( _contract_ )= _d2_


# Appendix B

# ByteString

Conceptually, a _ByteString_ is defined as a list of 8-bit words.

**datatype** ( _plugins del_ : _size_ ) _ByteString_ = _ByteString_ ( _8 word_ ) _list_

**definition** _emptyByteString_ :: _ByteString_ **where**
_emptyByteString_ = _ByteString_ []

**fun** _singletonByteString_ :: _8 word_ ⇒ _ByteString_ **where**
_singletonByteString w_ = _ByteString_ [ _w_ ]

**fun** _consByteString_ :: _8 word_ ⇒ _ByteString_ ⇒ _ByteString_ **where**
_consByteString w_ ( _ByteString t_ )= _ByteString_ ( _w_ # _t_ )

**fun** _appendByteStrings_ :: _ByteString_ ⇒ _ByteString_ ⇒ _ByteString_ **where**
_appendByteStrings_ ( _ByteString a_ )( _ByteString b_ )= _ByteString_ ( _a_ @ _b_ )

**fun** _innerListByteString_ :: _ByteString_ ⇒ _8 word list_ **where**
_innerListByteString_ ( _ByteString x_ )= _x_

**lemma** _lazyConsByteString_ : _consByteString w t_ = _ByteString_ ( _w_ # _innerList-
ByteString t_ )
**by** ( _metis consByteString_. _simps innerListByteString_. _elims_ )

**lemma** _intToWordToUint_ : _x_ ≥ _0_ =⇒ _x_ < _256_ =⇒ _uint_ ( _word-of-int x_ :: _8 word_ )
=( _x_ :: _int_ )
**apply** ( _simp only_ : _uint-word-of-int_ )
**by** _auto_

**lemma** _appendByteStringsAssoc_ : _appendByteStrings_ ( _appendByteStrings x y_ ) _z_


= _appendByteStrings x_ ( _appendByteStrings y z_ )
**by** ( _metis append_. _assoc appendByteStrings_. _simps innerListByteString_. _elims_ )

**fun** _lengthByteString_ :: _ByteString_ ⇒ _nat_ **where**
_lengthByteString_ ( _ByteString x_ )= _length x_

**fun** _takeByteString_ :: _nat_ ⇒ _ByteString_ ⇒ _ByteString_ **where**
_takeByteString n_ ( _ByteString x_ )= _ByteString_ ( _take n x_ )

**fun** _dropByteString_ :: _nat_ ⇒ _ByteString_ ⇒ _ByteString_ **where**
_dropByteString n_ ( _ByteString x_ )= _ByteString_ ( _drop n x_ )

**lemma** _appendTakeDropByteString_ : _appendByteStrings_ ( _takeByteString n x_ )( _dropByteString
n x_ )= _x_
**by** ( _metis appendByteStrings_. _simps append-take-drop-id dropByteString_. _simps
innerListByteString_. _cases takeByteString_. _simps_ )

The _BS_ helper allows to create a _ByteString_ out of a regular _string_.

**fun** _BS_ :: _string_ ⇒ _ByteString_ **where**
_BS str_ = _ByteString_ ( _map of-char str_ )

For example _BS_ ′′ _abc_ ′′is evaluated to _ByteString_ [ _97_ , _98_ , _99_ ]

**Size**

**instantiation** _ByteString_ :: _size_
**begin**

**definition** _size-ByteString_ **where**
_size-ByteString-overloaded-def_ : _size-ByteString_ = _lengthByteString_
**instance ..**

**end**

### B.1 Ordering

We define the(<)and(≤)functions that provide _ord_ ering.

**instantiation** _ByteString_ :: _ord_
**begin**

**fun** _less-eq-BS_ ′::( _8 word_ ) _list_ ⇒( _8 word_ ) _list_ ⇒ _bool_ **where**
_less-eq-BS_ ′ _Nil Nil_ = _True_ |


_less-eq-BS_ ′( _Cons - -_ ) _Nil_ = _False_ |
_less-eq-BS_ ′ _Nil_ ( _Cons - -_ )= _True_ |
_less-eq-BS_ ′( _Cons h1 t1_ )( _Cons h2 t2_ )=
( _if h2_ < _h1 then False
else_ ( _if h1_ = _h2 then less-eq-BS_ ′ _t1 t2 else True_ ))

**fun** _less-eq-BS_ :: _ByteString_ ⇒ _ByteString_ ⇒ _bool_ **where**
_less-eq-BS_ ( _ByteString xs_ )( _ByteString ys_ )= _less-eq-BS_ ′ _xs ys_

**definition** _a_ ≤ _b_ = _less-eq-BS a b_

**fun** _less-BS_ :: _ByteString_ ⇒ _ByteString_ ⇒ _bool_ **where**
_less-BS a b_ =(¬( _less-eq-BS b a_ ))

**definition** _a_ < _b_ = _less-BS a b_
**end**

And we also define some lemmas useful for total order.

**lemma** _oneLessEqBS_ ′:¬ _less-eq-BS_ ′ _bs2 bs1_ =⇒ _less-eq-BS_ ′ _bs1 bs2_
**lemma** _oneLessEqBS_ :¬ _less-eq-BS bs2 bs1_ =⇒ _less-eq-BS bs1 bs2_
**lemma** _less-eq-BS-trans_ ′: _less-eq-BS_ ′ _x y_ =⇒ _less-eq-BS_ ′ _y z_ =⇒ _less-eq-BS_ ′ _x z_
**lemma** _less-eq-BS-trans_ : _less-eq-BS x y_ =⇒ _less-eq-BS y z_ =⇒ _less-eq-BS x z_
**lemma** _byteStringLessEqTwiceEq_ ′: _less-eq-BS_ ′ _x y_ =⇒ _less-eq-BS_ ′ _y x_ =⇒ _x_ =
_y_
**lemma** _byteStringLessEqTwiceEq_ : _less-eq-BS x y_ =⇒ _less-eq-BS y x_ =⇒ _x_ = _y_
**lemma** _lineaBS_ : _less-eq-BS x y_ ∨ _less-eq-BS y x_


# Appendix C

# Code exports

This theory contains the necessary code to export a version of the Marlowe
Semantics in Haskell.

We start by importing the theories we want to export and a translation the-
ory. The theory _Code-Target-Numeral_ translates the default representation
of numbers (which is suitable for logic reasoning) into a more performant
representation.

**theory** _CodeExports_

**imports**
_Core_. _Semantics
Examples_. _Swap
HOL_ − _Library_. _Code-Target-Numeral
HOL_. _String_

**begin**

We provide some Serialization options to use Haskell native _String_ instead
of our logical represenation of _ByteString_

**code-printing**
— The first command tells the serializer to use Haskell
— native _String_ instead of our logical ByteString
**type-constructor** _ByteString_
⇀( _Haskell_ ) _String_
— The next three commands tells the serializer to use the operators provided by
— the Ord instance instead of the ones that work with the logical representation
| **constant** _less-eq-BS_
⇀( _Haskell_ ) **infix** _4_ <=
| **constant** _less-BS_


```
⇀( Haskell ) infix 4 <
| constant HOL. equal :: ByteString ⇒ ByteString ⇒ bool
⇀( Haskell ) infix 4 ==
— The next command tells the serializer to implode the logical Isabelle string
— into Haskell string. Because this is a textual rewrite, we need to force the
— generation of String.implode
| constant BS :: string ⇒ ByteString
⇀( Haskell ) Stringa. implode
```
With a **code_identifier** we hint what the name of the module should be.

**code-identifier
code-module** _Swap_ ⇀( _Haskell_ ) _Examples_. _Swap_

We export all the constants in one statement, because they don’t add up, if
you export two times, the second export will overwrite the first one.

**export-code**
— With the following exports, we declare that we want to have all the important
semantic functions. Ideally, just with this we would have everything we need, but
we need to force some exports.
_evalValue
evalObservation
reductionLoop
reduceContractUntilQuiescent
applyAllInputs
playTrace
computeTransaction_

```
— Export examples to be used as oracle specificaiton tests
swapExample
happyPathTransactions
happyPathPayments
```
— Force the export of string implode (works together with the BS code_printing
hint)
_String_. _implode_

```
— Force export of State record selectors
txOutContract
txOutWarnings
txOutPayments
txOutState
```
```
— Force export of Arith.Int constructor
```

```
int-of-integer
```
```
— Force export of TransactionOutput constructors
TransactionOutput
```
```
— Force export of TransactionWarning constructors
TransactionNonPositiveDeposit
```
```
— Force export of TransactionError constructors
TEAmbiguousTimeIntervalError
```
```
— Force export of Payment constructor
Payment
```
```
— Force the export of the transaction record
Transaction-ext
```
```
— Force the export of the transaction output record
TransactionOutputRecord-ext
```
— Force the export on some equality functions (sadly it does not force the Eq
instance)
_equal-TransactionWarning-inst_. _equal-TransactionWarning
equal-Payment-inst_. _equal-Payment
equal-Value-inst_. _equal-Value
equal-Observation-inst_. _equal-Observation
equal-Action-inst_. _equal-Action
equal-Input-inst_. _equal-Input
equal-Transaction-ext-inst_. _equal-Transaction-ext
equal-State-ext-inst_. _equal-State-ext
equal-IntervalError-inst_. _equal-IntervalError
equal-TransactionError-inst_. _equal-TransactionError
equal-TransactionOutput-inst_. _equal-TransactionOutput_

```
in Haskell ( string-classes )
```

# Appendix D

# Marlowe Core JSON

The Json specification for Marlowe Core is defined in Literate Haskell us-
ing the Aeson library. In order to fully understand the specification, some
knowledge of Haskell and the library is recommended but not necessary.

For each Marlowe datatype we define a way to parse the JSON into a value
(FromJSON instances) and a way to serialize a value to JSON (ToJSON
instances).

### D.1 Party.

Parties are serialized as a simple object with an _address_ or _role_token_ key,
depending on the _Party_ type.

```
instanceToJSON Partywhere
toJSON (Address address) =
object["address".=address]
toJSON (Role name) =
object["role_token".=name]
instanceFromJSON Partywhere
parseJSON =withObject"Party"$
λv→asAddress v<|>asRole v
where
asAddress v=Address<$>v.:"address"
asRole v=Role<$>v.:"role_token"
```
for example, the following _Party_

```
addressExample::Party
addressExample=Address"example address"
```

is serialized as{"address":"example address"}, and

```
roleExample::Party
roleExample=Role"example role"
```
is serialized as{"role_token":"example role"}

### D.2 Token

The _Token_ type is serialized as an object with two properties, _currency_sym-
bol_ and _token_name_

```
instanceToJSON Tokenwhere
toJSON (Token currSym tokName) =object
["currency_symbol".=currSym
,"token_name".=tokName
]
instanceFromJSON Tokenwhere
parseJSON =withObject"Token"
(λv→
Token<$>(v.:"currency_symbol")
<∗>(v.:"token_name")
)
```
for example, the following _Token_

```
dolarToken::Token
dolarToken=Token"85bb65" "dolar"
```
is serialized as{"currency_symbol":"85bb65","token_name":"dolar"}

### D.3 Payee

Payees are serialized as a simple object with an _account_ or _party_ key, de-
pending on the _Payee_ type.

```
instanceToJSON Payeewhere
toJSON (Account account) =
object["account".=account]
toJSON (Party party) =
```

```
object["party".=party]
instanceFromJSON Payeewhere
parseJSON =withObject"Payee"$
λv→asAccount v<|>asParty v
where
asAccount v=Account<$>v.:"account"
asParty v=Party<$>v.:"party"
```
for example, the following _Payee_

```
internalPayeeExample::Payee
internalPayeeExample=Account addressExample
```
is serialized as{"account":{"address":"example address"}}, and

```
externalPayeeExample::Payee
externalPayeeExample=Party roleExample
```
is serialized as{"party":{"role_token":"example role"}}

### D.4 ChoicesId

The _ChoiceId_ type is serialized as an object with two properties, _choice_name_
and _choice_owner_

```
instanceToJSON ChoiceIdwhere
toJSON (ChoiceId name party) =object
["choice_name".=name
,"choice_owner".=party
]
instanceFromJSON ChoiceIdwhere
parseJSON =withObject"ChoiceId"
(λv→
ChoiceId<$>(v.:"choice_name")
<∗>(v.:"choice_owner")
)
```
for example, the following _ChoiceId_

```
choiceIdExample::ChoiceId
choiceIdExample=ChoiceId"ada price"addressExample
```
is serialized as


##### {

```
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
}
```
### D.5 Bound

The _Bound_ type is serialized as an object with two properties, _from_ and _to_

```
instanceToJSON Boundwhere
toJSON (Bound from to) =object
["from".=from
,"to".=to
]
instanceFromJSON Boundwhere
parseJSON =withObject"Bound"(λv→
Bound<$>(getInteger"lower bound"=<<(v.:"from"))
<∗>(getInteger"higher bound"=<<(v.:"to"))
)
```
for example, the following _Bound_

```
exampleBound::Bound
exampleBound=Bound2 10
```
is serialized as{"from": 2,"to": 10}

### D.6 Values

The _ValueId_ type is serialized as a literal string.

```
instanceToJSON ValueIdwhere
toJSON (ValueId x) =toJSON x
instanceFromJSON ValueIdwhere
parseJSON =withText"ValueId"$return◦ValueId◦T.unpack
```
The _Value_ serialization depends on the constructor. A _Constant_ is serialized
as a _number_ , _TimeIntervalStart_ and _TimeIntervalEnd_ are serialized as literal


strings, and the rest are serialized as a single object (with keys depending on
the constructor).

```
instanceToJSON Valuewhere
toJSON (AvailableMoney accountId token) =object
["amount_of_token".=token
,"in_account".=accountId
]
toJSON (Constant(Int_of_integer x)) =toJSON x
toJSON (NegValue x) =object
["negate".=x]
toJSON (AddValue lhs rhs) =object
["add".=lhs
,"and".=rhs
]
toJSON (SubValue lhs rhs) =object
["value".=lhs
,"minus".=rhs
]
toJSON (MulValue lhs rhs) =object
["multiply".=lhs
,"times".=rhs
]
toJSON (DivValue lhs rhs) =object
["divide".=lhs
,"by".=rhs
]
toJSON (ChoiceValue choiceId) =object
["value_of_choice".=choiceId]
toJSON TimeIntervalStart=JSON.String$T.pack"time_interval_start"
toJSON TimeIntervalEnd=JSON.String$T.pack"time_interval_end"
toJSON (UseValue valueId) =object
["use_value".=valueId]
toJSON (Cond obs tv ev) =object
["if".=obs
,"then".=tv
,"else".=ev
]
instanceFromJSON Valuewhere
parseJSON (JSON.Object v) =
(AvailableMoney<$>(v.:"in_account")
```

```
<∗>(v.:"amount_of_token"))
<|>(NegValue<$>(v.:"negate"))
<|>(AddValue<$>(v.:"add")
<∗>(v.:"and"))
<|>(SubValue<$>(v.:"value")
<∗>(v.:"minus"))
<|>(MulValue<$>(v.:"multiply")
<∗>(v.:"times"))
<|>(DivValue<$>(v.:"divide")<∗>(v.:"by"))
<|>(ChoiceValue<$>(v.:"value_of_choice"))
<|>(UseValue<$>(v.:"use_value"))
<|>(Cond<$>(v.:"if")
<∗>(v.:"then")
<∗>(v.:"else"))
parseJSON (JSON.String"time_interval_start") =return TimeIntervalStart
parseJSON (JSON.String"time_interval_end") =return TimeIntervalEnd
parseJSON (JSON.Number n) =Constant<$>getInteger"constant value"n
parseJSON =fail"Value must be either a string, object or an integer"
```
Some examples for each _Value_ s type

**Constant**

```
constantExample::Value
constantExample=Constant 1
```
is serialized as 1

**TimeIntervalStart**

```
intervalStartExample::Value
intervalStartExample=TimeIntervalStart
```
is serialized as"time_interval_start"

**TimeIntervalEnd**

```
intervalEndExample::Value
intervalEndExample=TimeIntervalEnd
```
is serialized as"time_interval_end"


**AddValue**

```
addExample::Value
addExample=AddValue(Constant1) (Constant2)
```
is serialized as{"add": 1,"and": 2}

**SubValue**

```
subExample::Value
subExample=SubValue(Constant4) (Constant2)
```
is serialized as{"minus": 2,"value": 4}

**MulValue**

```
mulExample::Value
mulExample=MulValue(Constant3) (Constant6)
```
is serialized as{"multiply": 3,"times": 6}

**DivValue**

```
divExample::Value
divExample=DivValue(Constant8) (Constant4)
```
is serialized as{"by": 4,"divide": 8}

**NegValue**

```
negateExample::Value
negateExample=NegValue(Constant3)
```
is serialized as{"negate": 3}


**ChoiceValue**

```
choiceValueExample::Value
choiceValueExample=ChoiceValue choiceIdExample
```
is serialized as

```
{
"value_of_choice": {
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
}
}
```
**UseValue**

```
useValueExample::Value
useValueExample=UseValue(ValueId"variable name")
```
is serialized as{"use_value":"variable name"}

**Cond**

```
condExample::Value
condExample=Cond TrueObs addExample mulExample
```
is serialized as

```
{
"else": {
"multiply": 3,
"times": 6
},
"if": true,
"then": {
"add": 1,
"and": 2
}
}
```

**AvailableMoney**

```
availableMoneyExample::Value
availableMoneyExample=AvailableMoney addressExample dolarToken
```
is serialized as

```
{
"amount_of_token": {
"currency_symbol": "85bb65",
"token_name": "dolar"
},
"in_account": {
"address": "example address"
}
}
```
### D.7 Observation

The _Observation_ type is serialized as native boolean (for _TrueObs_ and _FalseObs_ )
or as an object with different properties, depending on the constructor.

```
instanceToJSON Observationwhere
toJSON (AndObs lhs rhs) =object
["both".=lhs
,"and".=rhs
]
toJSON (OrObs lhs rhs) =object
["either".=lhs
,"or".=rhs
]
toJSON (NotObs v) =object
["not".=v]
toJSON (ChoseSomething choiceId) =object
["chose_something_for".=choiceId]
toJSON (ValueGE lhs rhs) =object
["value".=lhs
,"ge_than".=rhs
]
toJSON (ValueGT lhs rhs) =object
```

```
["value".=lhs
,"gt".=rhs
]
toJSON (ValueLT lhs rhs) =object
["value".=lhs
,"lt".=rhs
]
toJSON (ValueLE lhs rhs) =object
["value".=lhs
,"le_than".=rhs
]
toJSON (ValueEQ lhs rhs) =object
["value".=lhs
,"equal_to".=rhs
]
toJSON TrueObs=toJSON True
toJSON FalseObs=toJSON False
instanceFromJSON Observationwhere
parseJSON (JSON.Bool True) =return TrueObs
parseJSON (JSON.Bool False) =return FalseObs
parseJSON (JSON.Object v) =
(AndObs<$>(v.:"both")
<∗>(v.:"and"))
<|>(OrObs<$>(v.:"either")
<∗>(v.:"or"))
<|>(NotObs<$>(v.:"not"))
<|>(ChoseSomething<$>(v.:"chose_something_for"))
<|>(ValueGE<$>(v.:"value")
<∗>(v.:"ge_than"))
<|>(ValueGT<$>(v.:"value")
<∗>(v.:"gt"))
<|>(ValueLT<$>(v.:"value")
<∗>(v.:"lt"))
<|>(ValueLE<$>(v.:"value")
<∗>(v.:"le_than"))
<|>(ValueEQ<$>(v.:"value")
<∗>(v.:"equal_to"))
parseJSON =fail"Observation must be either an object or a boolean"
```
Some examples for each _Observation_ type


**TrueObs**

```
trueExample::Observation
trueExample=TrueObs
```
is serialized astrue

**FalseObs**

```
falseExample::Observation
falseExample=FalseObs
```
is serialized asfalse

**AndObs**

```
andExample::Observation
andExample=AndObs TrueObs FalseObs
```
is serialized as{"and":false,"both":true}

**OrObs**

```
orExample::Observation
orExample=OrObs TrueObs FalseObs
```
is serialized as{"either":true,"or":false}

**NotObs**

```
notExample::Observation
notExample=NotObs TrueObs
```
is serialized as{"not":true}


**ChoseSomething**

```
choseExample::Observation
choseExample=ChoseSomething choiceIdExample
```
is serialized as

```
{
"chose_something_for": {
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
}
}
```
**ValueGE**

```
valueGEExample::Observation
valueGEExample=ValueGE(Constant1) (Constant2)
```
is serialized as{"ge_than": 2,"value": 1}

**ValueGT**

```
valueGTExample::Observation
valueGTExample=ValueGT(Constant1) (Constant2)
```
is serialized as{"gt": 2,"value": 1}

**ValueLT**

```
valueLTExample::Observation
valueLTExample=ValueLT (Constant1) (Constant2)
```
is serialized as{"lt": 2,"value": 1}


**ValueLE**

```
valueLEExample::Observation
valueLEExample=ValueLE(Constant1) (Constant2)
```
is serialized as{"le_than": 2,"value": 1}

**ValueEQ**

```
valueEQExample::Observation
valueEQExample=ValueEQ(Constant1) (Constant2)
```
is serialized as{"equal_to": 2,"value": 1}

### D.8 Action

The _Action_ type is serialized as an object with different properties, depending
the constructor.

```
instanceToJSON Actionwhere
toJSON (Deposit accountId party token val) =object
["into_account".=accountId
,"party".=party
,"of_token".=token
,"deposits".=val
]
toJSON (Choice choiceId bounds) =object
["for_choice".=choiceId
,"choose_between".=toJSONList(map toJSON bounds)
]
toJSON (Notify obs) =object
["notify_if".=obs]
instanceFromJSON Actionwhere
parseJSON =withObject"Action"(λv→
(Deposit<$>(v.:"into_account")
<∗>(v.:"party")
<∗>(v.:"of_token")
<∗>(v.:"deposits"))
<|>(Choice<$>(v.:"for_choice")
```

```
<∗>((v.:"choose_between")>>=
withArray"Bound list"(λbl→
mapM parseJSON (F.toList bl)
)))
<|>(Notify<$>(v.:"notify_if"))
)
```
Some examples for each _Action_ type

**Deposit**

```
depositExample::Action
depositExample=Deposit
addressExample
roleExample
dolarToken
constantExample
```
is serialized as

```
{
"deposits": 1,
"into_account": {
"address": "example address"
},
"of_token": {
"currency_symbol": "85bb65",
"token_name": "dolar"
},
"party": {
"role_token": "example role"
}
}
```
**Choice**

```
choiceExample::Action
choiceExample=Choice
choiceIdExample
[Bound0 1,Bound4 8]
```
is serialized as


##### {

```
"choose_between": [
{
"from": 0,
"to": 1
},
{
"from": 4,
"to": 8
}
],
"for_choice": {
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
}
}
```
**Notify**

```
notifyExample::Action
notifyExample=Notify(ChoseSomething choiceIdExample)
```
is serialized as

```
{
"notify_if": {
"chose_something_for": {
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
}
}
}
```
### D.9 Case

The _Case_ type is serialized as an object with two properties ( _case_ and _then_ ).


```
instanceToJSON Casewhere
toJSON (Case act cont) =object
["case".=act
,"then".=cont
]
instanceFromJSON Casewhere
parseJSON =withObject"Case"
(λv→
Case<$>(v.:"case")<∗>(v.:"then")
)
```
For example, the following _Case_

```
caseExample::Case
caseExample=Case notifyExample Close
```
is serialized as

```
{
"case": {
"notify_if": {
"chose_something_for": {
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
}
}
},
"then": "close"
}
```
### D.10 Contract

The _Contract_ type is serialized as the literal string "close" or as an object,
depending on the constructor

```
instanceToJSON Contractwhere
toJSON Close=JSON.String$T.pack"close"
toJSON (Pay accountId payee token value contract) =object
```

```
["from_account".=accountId
,"to".=payee
,"token".=token
,"pay".=value
,"then".=contract
]
toJSON (If obs cont1 cont2) =object
["if".=obs
,"then".=cont1
,"else".=cont2
]
toJSON (When caseList timeout cont) =object
["when".=toJSONList(map toJSON caseList)
,"timeout".=timeout
,"timeout_continuation".=cont
]
toJSON (Let valId value cont) =object
["let".=valId
,"be".=value
,"then".=cont
]
toJSON (Assert obs cont) =object
["assert".=obs
,"then".=cont
]
```
instanceFromJSON Contractwhere
parseJSON (JSON.String"close") =return Close
parseJSON (JSON.Object v) =
(Pay<$>(v.:"from_account")
<∗>(v.:"to")
<∗>(v.:"token")
<∗>(v.:"pay")
<∗>(v.:"then"))
<|>(If<$>(v.:"if")
<∗>(v.:"then")
<∗>(v.:"else"))
<|>(When<$>((v.:"when")>>=
withArray"Case list"(λcl→
mapM parseJSON(F.toList cl)
))


```
<∗>(withInteger"when timeout"=<<(v.:"timeout"))
<∗>(v.:"timeout_continuation"))
<|>(Let<$>(v.:"let")
<∗>(v.:"be")
<∗>(v.:"then"))
<|>(Assert<$>(v.:"assert")
<∗>(v.:"then"))
parseJSON =
fail"Contract must be either an object or a the string \"close\""
```
Some examples for each _Contract_ type

**Close**

```
closeExample::Contract
closeExample=Close
```
is serialized as"close"

**Pay**

```
payExample::Contract
payExample=Pay
roleExample
internalPayeeExample
dolarToken
(Constant10)
Close
```
is serialized as

```
{
"from_account": {
"role_token": "example role"
},
"pay": 10,
"then": "close",
"to": {
"account": {
"address": "example address"
```

##### }

##### },

```
"token": {
"currency_symbol": "85bb65",
"token_name": "dolar"
}
}
```
**If**

```
ifExample::Contract
ifExample=If
TrueObs
Close
Close
```
is serialized as

```
{
"else": "close",
"if": true,
"then": "close"
}
```
**When**

```
whenExample::Contract
whenExample=When
[Case(Notify TrueObs)Close
,Case(Notify FalseObs)Close
]
20
Close
```
is serialized as

```
{
"timeout": 20,
"timeout_continuation": "close",
"when": [
```

##### {

```
"case": {
"notify_if": true
},
"then": "close"
},
{
"case": {
"notify_if": false
},
"then": "close"
}
]
}
```
**Let**

```
letExample::Contract
letExample=Let(ValueId"var") (Constant10)Close
```
is serialized as

```
{
"be": 10,
"let": "var",
"then": "close"
}
```
**Assert**

```
assertExample::Contract
assertExample=Assert choseExample Close
```
is serialized as

```
{
"assert": {
"chose_something_for": {
"choice_name": "ada price",
"choice_owner": {
```

```
"address": "example address"
}
}
},
"then": "close"
}
```
### D.11 Input.

The _Input_ type is serialized as the literal string "input_notify" or as an object,
depending on the constructor.

```
instanceToJSON Inputwhere
toJSON (IDeposit accId party tok amount) =object
["input_from_party".=party
,"that_deposits".=amount
,"of_token".=tok
,"into_account".=accId
]
toJSON (IChoice choiceId chosenNum) =object
["input_that_chooses_num".=chosenNum
,"for_choice_id".=choiceId
]
toJSON INotify=JSON.String$T.pack"input_notify"
instanceFromJSON Inputwhere
parseJSON (JSON.String"input_notify") =return INotify
parseJSON (JSON.Object v) =
IChoice<$>v.:"for_choice_id"
<∗>v.:"input_that_chooses_num"
<|>IDeposit<$>v.:"into_account"
<∗>v.:"input_from_party"
<∗>v.:"of_token"
<∗>v.:"that_deposits"
parseJSON =
fail"Input must be either an object or the string \"input_notify\""
```
Some examples for each _Input_ type


**INotify**

```
iNotifyExample::Input
iNotifyExample=INotify
```
is serialized as"input_notify"

**IChoice**

```
iChoiceExample::Input
iChoiceExample=IChoice choiceIdExample 3
```
is serialized as

```
{
"for_choice_id": {
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
},
"input_that_chooses_num": 3
}
```
**IDeposit**

```
iDepositExample::Input
iDepositExample=IDeposit addressExample roleExample dolarToken 5
```
is serialized as

```
{
"input_from_party": {
"role_token": "example role"
},
"into_account": {
"address": "example address"
},
"of_token": {
"currency_symbol": "85bb65",
```

```
"token_name": "dolar"
},
"that_deposits": 5
}
```
### D.12 Transaction

The _Transaction_ type is serialized as an object with two properties, _tx_in-
terval_ and _tx_inputs_.

```
instanceToJSON(Transaction_ext a)where
toJSON (Transaction_ext(from,to)txInps ) =object
["tx_interval".=timeIntervalJSON
,"tx_inputs".=toJSONList(map toJSON txInps)
]
wheretimeIntervalJSON =object["from".=from
,"to".=to
]
instanceFromJSON(Transaction_ext())where
parseJSON (JSON.Object v) =
Transaction_ext<$>(parseTimeInterval=<<(v.:"tx_interval"))
<∗>((v.:"tx_inputs")>>=
withArray"Transaction input list"(λcl→
mapM parseJSON(F.toList cl)
))
<∗>pure()
whereparseTimeInterval=withObject"TimeInterval"(λv→
dofrom←withInteger"TimeInterval from"=<<(v.:"from")
to←withInteger"TimeInterval to"=<<(v.:"to")
return(from,to)
)
parseJSON =fail"Transaction must be an object"
```
for example, the following _Transaction_

```
transactionExample::Transaction_ext()
transactionExample=Transaction_ext
(10,100)
[iChoiceExample
,iNotifyExample
```

##### ]

##### ()

is serialized as

```
{
"tx_inputs": [
{
"for_choice_id": {
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
},
"input_that_chooses_num": 3
},
"input_notify"
],
"tx_interval": {
"from": 10,
"to": 100
}
}
```
### D.13 Payment

The _Payment_ type is serialized as a single object with three properties

```
instanceToJSON Paymentwhere
toJSON (Payment from to token amount) =object
["payment_from".=from
,"to".=to
,"token".=token
,"amount".=amount
]
instanceFromJSON Paymentwhere
parseJSON =withObject"Payment"
(λv→
Payment<$>(v.:"payment_from")
<∗>(v.:"to")
```

```
<∗>(v.:"token")
<∗>(v.:"amount")
)
```
for example, the following _Payment_

```
paymentExample::Payment
paymentExample=Payment
addressExample
externalPayeeExample
dolarToken
10
```
is serialized as

```
{
"amount": 10,
"payment_from": {
"address": "example address"
},
"to": {
"party": {
"role_token": "example role"
}
},
"token": {
"currency_symbol": "85bb65",
"token_name": "dolar"
}
}
```
### D.14 State

The _State_ type is serialized as a single object with four properties. Each Map
is represented by a list of key value tuples.

```
instanceToJSON(State_ext())where
toJSON (State_ext accounts choices boundValues minTime ) =object
["accounts".=toJSON accounts
,"choices".=toJSON choices
,"boundValues".=toJSON boundValues
```

```
,"minTime".=minTime
]
instanceFromJSON(State_ext())where
parseJSON =withObject"State"
(λv→
State_ext<$>(v.:"accounts")
<∗>(v.:"choices")
<∗>(v.:"boundValues")
<∗>(v.:"minTime")
<∗>pure()
)
```
for example, the following state

```
stateExample::State_ext()
stateExample=State_ext
[((roleExample,dolarToken),20)]
[(choiceIdExample,10)]
[(ValueId"example",30)]
90
()
```
is serialized as

```json
{
"accounts": [
[
[
{
"role_token": "example role"
},
{
"currency_symbol": "85bb65",
"token_name": "dolar"
}
],
20
]
],
"boundValues": [
[
```

```json
"example",
30
]
],
"choices": [
[
{
"choice_name": "ada price",
"choice_owner": {
"address": "example address"
}
},
10
]
],
"minTime": 90
}
```
### D.15 TransactionWarning.

The _TransactionWarning_ type is serialized as a literal string (in case of _Trans-
actionAssertionFailed_ ) or as an object with different properties, depending
the constructor.

```
instanceToJSON TransactionWarningwhere
toJSON (TransactionNonPositiveDeposit party accId tok amount) =object
["party".=party
,"asked_to_deposit".=amount
,"of_token".=tok
,"in_account".=accId
]
toJSON (TransactionNonPositivePay accId payee tok amount) =object
["account".=accId
,"asked_to_pay".=amount
,"of_token".=tok
,"to_payee".=payee
]
toJSON (TransactionPartialPay accId payee tok paid expected) =object
["account".=accId
,"asked_to_pay".=expected
```

```
,"of_token".=tok
,"to_payee".=payee
,"but_only_paid".=paid
]
toJSON (TransactionShadowing valId oldVal newVal) =object
["value_id".=valId
,"had_value".=oldVal
,"is_now_assigned".=newVal
]
toJSON TransactionAssertionFailed=JSON.String$T.pack"assertion_failed"
instanceFromJSON TransactionWarningwhere
parseJSON (JSON.String"assertion_failed") =
return TransactionAssertionFailed
parseJSON (JSON.Object v) =
(TransactionNonPositiveDeposit<$>(v.:"party")
<∗>(v.:"in_account")
<∗>(v.:"of_token")
<∗>(v.:"asked_to_deposit"))
<|>(domaybeButOnlyPaid←v.:?"but_only_paid"
casemaybeButOnlyPaid::Maybe Scientificof
Nothing→TransactionNonPositivePay<$>(v.:"account")
<∗>(v.:"to_payee")
<∗>(v.:"of_token")
<∗>(v.:"asked_to_pay")
Just butOnlyPaid→TransactionPartialPay<$>(v.:"account")
<∗>(v.:"to_payee")
<∗>(v.:"of_token")
<∗>getInteger"but only paid"butOnlyPaid
<∗>(v.:"asked_to_pay"))
<|>(TransactionShadowing<$>(v.:"value_id")
<∗>(v.:"had_value")
<∗>(v.:"is_now_assigned"))
parseJSON =
fail"Contract must be either an object or a the string \"close\""
```
Some examples for each _TransactionWarning_ type

**TransactionNonPositiveDeposit**

```
transactionNonPositiveDepositExample::TransactionWarning
transactionNonPositiveDepositExample=TransactionNonPositiveDeposit
```

```
addressExample
roleExample
dolarToken
20
```
is serialized as

```json
{
  "asked_to_deposit": 20,
  "in_account": {
    "role_token": "example role"
  },
  "of_token": {
    "currency_symbol": "85bb65",
    "token_name": "dolar"
  },
  "party": {
    "address": "example address"
  }
}
```
**TransactionNonPositivePay**

```haskell
transactionNonPositivePayExample::TransactionWarning
transactionNonPositivePayExample=TransactionNonPositivePay
addressExample
internalPayeeExample
dolarToken
20
```
is serialized as

```json
{
  "account": {
  "address": "example address"
  },
  "asked_to_pay": 20,
  "of_token": {
  "currency_symbol": "85bb65",
  "token_name": "dolar"
  },
  "to_payee": {
    "account": {
      "address": "example address"
    }
  }
}
```
**TransactionPartialPay**

```
transactionPartialPayExample::TransactionWarning
transactionPartialPayExample=TransactionPartialPay
addressExample
internalPayeeExample
dolarToken
20
30
```
is serialized as

```json
{
  "account": {
    "address": "example address"
  },
  "asked_to_pay": 30,
  "but_only_paid": 20,
  "of_token": {
  "currency_symbol": "85bb65",
  "token_name": "dolar"
  },
  "to_payee": {
    "account": {
      "address": "example address"
    }
  }
}
```
**TransactionShadowing**

```
transactionShadowingExample::TransactionWarning
transactionShadowingExample=TransactionShadowing
```

```haskell
(ValueId"example")
4
5
```
is serialized as

```json
{
  "had_value": 4,
  "is_now_assigned": 5,
  "value_id": "example"
}
```
**TransactionAssertionFailed**

```
transactionAssertionFailedExample::TransactionWarning
transactionAssertionFailedExample=TransactionAssertionFailed
```
is serialized as"assertion_failed"

## D.16 IntervalError

The _IntervalError_ type is serialized as an object with a single property (de-
pending on the constructor) and in a tuple, the values.

```haskell
instanceToJSON IntervalErrorwhere
toJSON (InvalidInterval(s,e)) =object
[("invalidInterval".=toJSON(s,e))]
toJSON (IntervalInPastError t(s,e)) =object
[("intervalInPastError".=toJSON (t,s,e))]
instanceFromJSON IntervalErrorwhere
parseJSON (JSON.Object v) =
let
parseInvalidInterval=do
(s,e)←v.:"invalidInterval"
pure$InvalidInterval(s,e)
parseIntervalInPastError=do
(t,s,e)←v.:"intervalInPastError"
pure$IntervalInPastError t(s,e)
in
parseIntervalInPastError<|>parseInvalidInterval
```

```
parseJSON invalid=
JSON.prependFailure"parsing IntervalError failed, "(JSON.typeMismatch"Object"invalid)
```
Some examples for each _IntervalError_ type

**InvalidInterval**

```
invalidIntervalExample::IntervalError
invalidIntervalExample=InvalidInterval(10,20)
```
is serialized as{"invalidInterval": [10,20]}

**IntervalInPastError**

```
intervalInPastErrorExample::IntervalError
intervalInPastErrorExample=IntervalInPastError30 (10,20)
```
is serialized as{"intervalInPastError": [30, 10 ,20]}

## D.17 TransactionError

The _TransactionError_ type is serialized as an object with a _tag_ property that
differentiates the type, and a _contents_ property that includes the parameter
if any.

```haskel
instanceToJSON TransactionErrorwhere
toJSON TEAmbiguousTimeIntervalError=object
["tag".=JSON.String"TEAmbiguousTimeIntervalError"
,"contents".=JSON.Null
]
toJSON TEApplyNoMatchError=object
["tag".=JSON.String"TEApplyNoMatchError"
,"contents".=JSON.Null
]
toJSON (TEIntervalError e) =object
["tag".=JSON.String"TEIntervalError"
,"contents".=toJSON e
]
toJSON TEUselessTransaction=object
```

```haskell
["tag".=JSON.String"TEUselessTransaction"
,"contents".=JSON.Null
]
instanceFromJSON TransactionErrorwhere
parseJSON=withObject"TransactionError"
(λv→
do
tag::String←v.:"tag"
casetagof
"TEAmbiguousTimeIntervalError"→
pure TEAmbiguousTimeIntervalError
"TEApplyNoMatchError"→
pure TEApplyNoMatchError
"TEIntervalError"→
TEIntervalError<$>v.:"contents"
"TEUselessTransaction"→
pure TEUselessTransaction
)
```
Some examples for each _TransactionError_ type

**TEAmbiguousTimeIntervalError**

```
teAmbiguousTimeIntervalErrorExample::TransactionError
teAmbiguousTimeIntervalErrorExample=TEAmbiguousTimeIntervalError
```
is serialized as

```json
{
  "contents": null,
  "tag": "TEAmbiguousTimeIntervalError"
}
```
**TEApplyNoMatchError**

```
teApplyNoMatchErrorExample::TransactionError
teApplyNoMatchErrorExample=TEApplyNoMatchError
```
is serialized as


##### {

```json
  "contents": null,
  "tag": "TEApplyNoMatchError"
}
```
**TEIntervalError**

```
teIntervalErrorExample::TransactionError
teIntervalErrorExample=TEIntervalError intervalInPastErrorExample
```
is serialized as

```json
{
  "contents": {
    "intervalInPastError": [
      30,
      10,
      20
    ]
  },
  "tag": "TEIntervalError"
  }
```
**TEUselessTransaction**

```haskell
teUselessTransactionExample::TransactionError
teUselessTransactionExample=TEUselessTransaction
```
is serialized as

```json
{
  "contents": null,
  "tag": "TEUselessTransaction"
}
```
## D.18 TransactionOutput

The _TransactionOutput_ is serialized as a single object with one property
( _transaction_error_ ) in case of an error, or 4 properties in case of success.


```haskell
instanceToJSON TransactionOutputwhere
toJSON (TransactionError err) =object
["transaction_error".=toJSON err]
toJSON (TransactionOutput out)
=object
["warnings".=toJSON(txOutWarnings out)
,"payments".=toJSON(txOutPayments out)
,"state".=toJSON (txOutState out)
,"contract".=toJSON(txOutContract out)
]
instanceFromJSON TransactionOutputwhere
parseJSON =withObject"TransactionOutput"
(λv→
(TransactionError<$>(v.:"transaction_error"))
<|>(TransactionOutput<$>
(TransactionOutputRecord_ext
<$>(v.:"warnings")
<∗>(v.:"payments")
<∗>(v.:"state")
<∗>(v.:"contract")
<∗>pure()
)
)
)
```
Some examples for each _TransactionOutput_ type

**TransactionError**

```
transactionOutputErrorExample::TransactionOutput
transactionOutputErrorExample=TransactionError teUselessTransactionExample
```
is serialized as

```json
{
"transaction_error": {
  "contents": null,
  "tag": "TEUselessTransaction"
  }
}
```

**TransactionOutput**

```
transactionOutputSuccessExample::TransactionOutput
transactionOutputSuccessExample=playTrace
0
Examples.Swap.swapExample
Examples.Swap.happyPathTransactions
```
is serialized as

```json
{
"contract": "close",
"payments": [
{
"amount": 10,
"payment_from": {
"role_token": "Ada Provider"
},
"to": {
"party": {
"role_token": "Dollar Provider"
}
},
"token": {
"currency_symbol": "",
"token_name": ""
}
},
{
"amount": 20,
"payment_from": {
"role_token": "Dollar Provider"
},
"to": {
"party": {
"role_token": "Ada Provider"
}
},
"token": {
"currency_symbol": "85bb65",
"token_name": "dollar"
 }
 }
],
"state": {
"accounts": [],
"boundValues": [],
"choices": [],
"minTime": 1664812900000
},
"warnings": []
}
```

## D.19 Full Contract Example

The Swap Example, defined in section §A.1.2is serialized as

```
{
"timeout": 1664812800000,
"timeout_continuation": "close",
"when": [
{
"case": {
"deposits": 10,
"into_account": {
"role_token": "Ada Provider"
},
"of_token": {
"currency_symbol": "",
"token_name": ""
},
"party": {
"role_token": "Ada Provider"
}
},
"then": {
"timeout": 1664816400000,
"timeout_continuation": "close",
"when": [
{
"case": {
"deposits": 20,
"into_account": {
"role_token": "Dollar Provider"
},
"of_token": {
"currency_symbol": "85bb65",
"token_name": "dollar"
},
"party": {
"role_token": "Dollar Provider"
}
},
"then": {
"from_account": {
"role_token": "Ada Provider"
},
"pay": 10,
"then": {
"from_account": {
"role_token": "Dollar Provider"
},
"pay": 20,
"then": "close",
"to": {
"party": {
"role_token": "Ada Provider"
}
},
"token": {
"currency_symbol": "85bb65",
"token_name": "dollar"
}
},
"to": {
"party": {
"role_token": "Dollar Provider"
}
},
"token": {
"currency_symbol": "",
"token_name": ""
 } } } ] } } ] }
```

## D.20 Parse utils

These are some Aeson utils to help parse a number to the Isabelle exported
_Arithİnt_

```
getInteger::String→Scientific→Parser Arith.Int
getInteger ctx x=case(floatingOrInteger x::Either Double Integer)of
Right a→return$Int_of_integer a
Left →fail$"parsing "++ctx++" failed, expected integer, but encountered floating point"
withInteger::String→JSON.Value→Parser Arith.Int
withInteger ctx=withScientific ctx$getInteger ctx
instanceToJSON Arith.Intwhere
toJSON (Int_of_integer x) =toJSON x
instanceFromJSON Arith.Intwhere
parseJSON (JSON.Number x) =getInteger"Int"x
parseJSON =fail"expecting integer"
```


import { abbreviateInteger, numberAsPercentage, numberAsSize } from "../utils/util";

interface Props {
  title: string;
  value: string;
}

interface NumericProps {
  title: string;
  value: number;
  digits: number;
}

export function SingleStat(props: Props) {
  return (
    <div class="level-item has-text-centered">
      <div>
        <p class="heading is-size-5">{props.title}</p>
        <p class="title">{props.value}</p>
      </div>
    </div>
  );
}

export function AbbreviatedSingleStat(props: NumericProps) {
  return <SingleStat title={props.title} value={abbreviateInteger(props.value, props.digits)} />;
}

export function PercentageSingleStat(props: NumericProps) {
  return <SingleStat title={props.title} value={numberAsPercentage(props.value, props.digits)} />;
}

export function SizeSingleStat(props: NumericProps) {
  return <SingleStat title={props.title} value={numberAsSize(props.value, props.digits)} />;
}

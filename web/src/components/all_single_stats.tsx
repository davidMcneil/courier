import { CourierState } from "../utils/data_parsers";
import { AbbreviatedSingleStat, PercentageSingleStat, SizeSingleStat } from "./single_stat";

interface Props {
  courierState: CourierState;
}

export function AllSingleStats(props: Props) {
  const s = props.courierState;
  return (
    <section class="section">
      <div class="container">
        {/* Current Stats */}
        <div class="level">
          <h1 class="is-size-5">Current</h1>
          <AbbreviatedSingleStat title="Topics" value={s.topics} digits={1} />
          <AbbreviatedSingleStat title="Subscriptions" value={s.subscriptions} digits={1} />
          <AbbreviatedSingleStat title="Messages" value={s.messages} digits={1} />
          <AbbreviatedSingleStat title="Pending" value={s.pending} digits={1} />
          <PercentageSingleStat title="Processed" value={s.percentageProcessed} digits={1} />
          <SizeSingleStat title="Memory RSS" value={s.memoryResidentSetSize} digits={1} />
        </div>

        {/* Interval Stats */}
        <div class="level">
          <h1 class="level-left">Interval</h1>
          <AbbreviatedSingleStat title="Topics" value={s.topicsInterval} digits={1} />
          <AbbreviatedSingleStat title="Subscriptions" value={s.subscriptionsInterval} digits={1} />
          <AbbreviatedSingleStat title="Messages" value={s.messagesInterval} digits={1} />
          <AbbreviatedSingleStat title="Expired" value={s.expiredInterval} digits={1} />
          <AbbreviatedSingleStat title="Pulled" value={s.pulledInterval} digits={1} />
          <AbbreviatedSingleStat title="Retries" value={s.pulledRetriesInterval} digits={1} />
          <AbbreviatedSingleStat title="Acks" value={s.acksInterval} digits={1} />
          <AbbreviatedSingleStat title="Acked" value={s.ackedInterval} digits={1} />
          <SizeSingleStat title="Memory RSS" value={s.memoryResidentSetSizeInterval} digits={1} />
        </div>
        {/* All Time Stats */}
        <div class="level">
          <h1 class="level-left">All Time</h1>
          <AbbreviatedSingleStat title="Topics" value={s.topicsAllTime} digits={1} />
          <AbbreviatedSingleStat title="Subscriptions" value={s.subscriptionsAllTime} digits={1} />
          <AbbreviatedSingleStat title="Messages" value={s.messagesAllTime} digits={1} />
          <AbbreviatedSingleStat title="Expired" value={s.expiredAllTime} digits={1} />
          <AbbreviatedSingleStat title="Pulled" value={s.pulledAllTime} digits={1} />
          <AbbreviatedSingleStat title="Retries" value={s.pulledRetriesAllTime} digits={1} />
          <AbbreviatedSingleStat title="Acks" value={s.acksAllTime} digits={1} />
          <AbbreviatedSingleStat title="Acked" value={s.ackedInterval} digits={1} />
        </div>
      </div>
    </section>
  );
}

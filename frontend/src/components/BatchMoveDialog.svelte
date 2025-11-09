<script>
  /**
   * Batch Move & Copy Integration
   * Handles the complete flow: selection -> destination -> operation -> progress
   * 
   * Usage:
   *   <BatchMoveDialog bind:this={batchMoveDialog} />
   *   
   *   // In your component:
   *   batchMoveDialog.open('move', selectedFiles);
   *   batchMoveDialog.open('copy', selectedFiles);
   */

  import { onMount } from 'svelte';
  import DestinationPicker from './ui/DestinationPicker.svelte';
  import BatchOperationDialog from './ui/BatchOperationDialog.svelte';
  import { batchOperationManager, OPERATION_TYPE } from '../lib/batchOperations';
  import { toast } from '../stores/ui';
  import { t } from '../lib/i18n';

  let destinationPicker = $state(null);
  let batchDialog = $state(null);
  let selectedItems = $state([]);
  let operationType = $state('copy');

  export function open(type, items) {
    if (!items || items.length === 0) {
      toast.show(t('batch.noItemsSelected'), 'error');
      return;
    }

    selectedItems = items;
    operationType = type;

    // Open destination picker
    destinationPicker?.open?.('/');
  }

  async function handleDestinationSelected(destination) {
    try {
      // Start batch operation
      const operation = await batchOperationManager.startOperation(
        operationType === 'move' ? OPERATION_TYPE.MOVE : OPERATION_TYPE.COPY,
        selectedItems,
        destination
      );

      // Open batch operation dialog with operation details
      batchDialog?.open?.(
        operationType,
        operation.items,
        destination
      );

      // Dispatch event for UI refresh
      window.dispatchEvent(new CustomEvent('batch-operation-started', { 
        detail: operation 
      }));
    } catch (error) {
      console.error('Error starting batch operation:', error);
      toast.show(t('errors.operationFailed'), 'error');
    }
  }

  function handleDestinationCancelled() {
    selectedItems = [];
  }

  function handleBatchOperationComplete(event) {
    const { type, destination, processedItems, failedItems } = event.detail;

    // Show completion message
    if (failedItems.length === 0) {
      toast.show(
        type === 'move'
          ? t('batch.moveComplete', { count: processedItems })
          : t('batch.copyComplete', { count: processedItems }),
        'success'
      );
    } else {
      toast.show(
        t('batch.operationPartiallyFailed', {
          success: processedItems - failedItems.length,
          failed: failedItems.length
        }),
        'warning'
      );
    }

    // Dispatch event for UI refresh
    window.dispatchEvent(new CustomEvent('batch-operation-complete', {
      detail: { type, destination, processedItems, failedItems }
    }));

    // Reset
    selectedItems = [];
  }
</script>

<!-- Destination Picker Modal -->
<DestinationPicker
  bind:this={destinationPicker}
  onSelect={handleDestinationSelected}
  onCancel={handleDestinationCancelled}
/>

<!-- Batch Operation Dialog -->
<BatchOperationDialog
  bind:this={batchDialog}
  onComplete={handleBatchOperationComplete}
/>
